use anyhow::{anyhow, bail, Result};
use clients::base_client;
use folders::hub_fold::make_hub;
use futures::{stream, StreamExt};
use serde::{de::DeserializeOwned, Deserialize};
use sha1::{Digest, Sha1};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    time::Duration,
    env::consts::{OS, ARCH}
};
use tokio::{fs, io::AsyncWriteExt, sync::Mutex};

const INDEX: &str = "https://launchermeta.mojang.com/v1/products/java-runtime/2ec0cc96c44e5a76b9c8b7c39df7210883d12871/all.json";
const CONCURRENCY: usize = 16;
const RETRIES: u32 = 3;
const MARKER: &str = ".complete";

static LOCK: Mutex<()> = Mutex::const_new(());

type Platform = HashMap<String, Vec<RuntimeEntry>>;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Index {
     #[serde(default)]
     pub windows_x64: Platform,
     #[serde(default)]
     pub windows_arm64: Platform,
     #[serde(default)]
     pub windows_x86: Platform,
     #[serde(default)]
     pub linux: Platform,
     #[serde(default)]
     pub linux_i386: Platform,
     #[serde(default)]
     pub mac_os: Platform,
     #[serde(default)]
     pub mac_os_arm64: Platform,
}

impl Index {
     fn machine(self) -> Result<Platform> {
          Ok(match (OS, ARCH) {
               ("windows", "x86_64") => self.windows_x64,
               ("windows", "aarch64") => self.windows_arm64,
               ("windows", "x86") => self.windows_x86,
               ("linux", "x86_64") => self.linux,
               ("linux", "x86") => self.linux_i386,
               ("macos", "x86_64") => self.mac_os,
               ("macos", "aarch64") => self.mac_os_arm64,
               (os, arch) => bail!("no java runtime found for: {}/{}", os, arch),
          })
     }
}

#[derive(Deserialize)]
pub struct RuntimeEntry {
     pub manifest: ManiRef,
}

#[derive(Deserialize)]
pub struct ManiRef {
     pub url: String,
}

#[derive(Deserialize)]
pub struct Mani {
     pub files: HashMap<String, FileEntry>,
}

#[derive(Deserialize)]
pub struct FileEntry {
     #[serde(rename = "type")]
     pub kind: String,
     pub downloads: Option<Downloads>,
     #[serde(default)]
     pub executable: bool,
     pub target: Option<String>
}

#[derive(Deserialize)]
pub struct Downloads {
     pub raw: Raw,
}

#[derive(Deserialize)]
pub struct Raw {
     pub sha1: String,
     pub url: String,
}

fn java_rel() -> &'static str {
     if cfg!(windows) {
          "bin/java.exe"
     } else if cfg!(target_os = "macos") {
          "jre.bundle/Contents/Home/bin/java"
     } else {
          "bin/java"
     }
}

async fn get<T: DeserializeOwned>(client: &reqwest::Client, url: &str) -> Result<T> {
     Ok(client.get(url).send().await?.error_for_status()?.json().await?)
}

pub async fn ensure(component: &str) -> Result<PathBuf> {
     let home = make_hub()?;
     let dir = home.join("java").join(component);
     let java = dir.join(java_rel());
     if dir.join(MARKER).exists() {
          return Ok(java);
     }
     let _guard = LOCK.lock().await;
     if dir.join(MARKER).exists() {
          return Ok(java);
     }

     let client = base_client();
     let index: Index = get(&client, INDEX).await?;
     let entry = index.machine()?.remove(component).and_then(|bu| bu.into_iter().next()).ok_or_else(|| anyhow!("no component {}", component))?;
     let mani: Mani = get(&client, &entry.manifest.url).await?;

     let mut jobs: Vec<(PathBuf, Raw, bool)> = Vec::new();
     let mut links: Vec<(PathBuf, String)> = Vec::new();
     for (rel, f) in mani.files {
          let path = dir.join(&rel);
          match f.kind.as_str() {
               "directory" => fs::create_dir_all(&path).await?,
               "file" => { let raw = f.downloads.ok_or_else(|| anyhow!("{}: no download", rel))?.raw; jobs.push((path, raw, f.executable)); }
               "link" => { if let Some(target) = f.target { links.push((path, target)); }}
               _ => {}
          }
     }
     let results: Vec<Result<()>> = stream::iter(jobs).map(|(path, raw, exec)| {
          let client = client.clone();
          async move {
               download_with_retry(&client, &raw, &path, exec).await?;
               Ok::<(), anyhow::Error>(())
          }
     }).buffer_unordered(CONCURRENCY).collect().await;

     for r in results {
          r?;
     }

     #[cfg(unix)]
     for (path, target) in links {
          if let Some(parent) = path.parent() {
               fs::create_dir_all(parent).await?;
          }
          let _ = fs::remove_file(&path).await;
          std::os::unix::fs::symlink(target, &path)?;
     }

     #[cfg(not(unix))]
     let _ = links;

     fs::write(dir.join(MARKER), "").await?;
     Ok(java)
}

async fn download_with_retry(client: &reqwest::Client, raw: &Raw, dest: &Path, executable: bool) -> Result<()> {
     let mut last = anyhow!("no attempts made");
     for attempt in 1..=RETRIES {
          match download_file(client, raw, dest, executable).await {
               Ok(()) => return Ok(()),
               Err(e) => {
                    last = e;
                    tokio::time::sleep(Duration::from_millis(500 * attempt as u64)).await;
               }
          }
     }
     Err(last.context(format!("downloading: {}", raw.url)))
}

async fn download_file(client: &reqwest::Client, raw: &Raw, dest: &Path, executable: bool) -> Result<()> {
     if let Some(parent) = dest.parent() {
          fs::create_dir_all(parent).await?;
     }
     if let Ok(existing) = fs::read(dest).await {
          let mut h = Sha1::new();
          h.update(&existing);
          if hex::encode(h.finalize()) == raw.sha1 {
               return set_exec(dest, executable).await;
          }
     }

     let mut tmp = dest.as_os_str().to_owned();
     tmp.push(".part");
     let tmps = PathBuf::from(tmp);
     let resp = client.get(&raw.url).send().await?.error_for_status()?;
     let mut body = resp.bytes_stream();
     let mut file = fs::File::create(&tmps).await?;
     let mut hasher = Sha1::new();

     while let Some(chunk) = body.next().await {
          let chunk = chunk?;
          hasher.update(&chunk);
          file.write_all(&chunk).await?;
     }
     file.flush().await?;
     drop(file);
     if hex::encode(hasher.finalize()) != raw.sha1 {
          let _ = fs::remove_file(&tmps).await;
          bail!("sha1 mismatch {}", raw.url);
     }

     fs::rename(&tmps, dest).await?;
     set_exec(dest, executable).await
}

#[cfg(unix)]
async fn set_exec(path: &Path, executable: bool) -> Result<()> {
     if executable {
          use std::os::unix::fs::PermissionsExt;
          fs::set_permissions(path, std::fs::Permissions::from_mode(0o755)).await?;
     }
     Ok(())
}

#[cfg(not(unix))]
async fn set_exec(path: &Path, executable: bool) -> Result<()> {
    Ok(())
}