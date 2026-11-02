use std::{collections::HashMap, env::consts::{ARCH, OS}, path::{Component, Path, PathBuf}, time::Duration, sync::atomic::{AtomicU64, Ordering}};
use futures::{stream, StreamExt};
use utils::{client::base, folders::make_hub};
use anyhow::{Context, Result, anyhow, bail};
use serde::{Deserialize, de::DeserializeOwned};
use sha1::{Digest, Sha1};
use tokio::{sync::Mutex, fs, fs::OpenOptions, io::AsyncWriteExt};

const INDEX: &str = "https://launchermeta.mojang.com/v1/products/java-runtime/2ec0cc96c44e5a76b9c8b7c39df7210883d12871/all.json";
const CONCURRENCY: usize = 10;
const RETRIES: u32 = 3;
const MARKER: &str = ".complete";
static LOCK: Mutex<()> = Mutex::const_new(());
type Platform = HashMap<String, Vec<RuntimeEntry>>;
static TEMP_ID: AtomicU64 = AtomicU64::new(0);

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
               (os, arch) => bail!("no java runtime found for {}/{}", os, arch),
          })
     }
}

#[derive(Deserialize)]
pub struct RuntimeEntry {
     pub manifest: ManifestRef,
}

#[derive(Deserialize)]
pub struct ManifestRef {
     pub url: String,
}

#[derive(Deserialize)]
pub struct Manifest {
     pub files: HashMap<String, FileEntry>,
}

#[derive(Deserialize)]
pub struct FileEntry {
     pub kind: String,
     pub downloads: Option<Downloads>,
     pub executable: bool,
     pub target: Option<String>,
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

fn valid_component(component: &str) -> bool {
     !component.is_empty() && component.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
}

fn safe_mani(root: &Path, relative: &str) -> Result<PathBuf> {
     let relative = Path::new(relative);
     if relative.as_os_str().is_empty() || relative.components().any(|p| !matches!(p, Component::Normal(_))) {
          bail!("unsafe runtime mani path: {}", relative.display());
     }
     Ok(root.join(relative))
}

pub async fn ensure(component: &str) -> Result<PathBuf> {
     if !valid_component(component) {
          bail!("invalid Java runtime component: {}", component);
     }
     let home = make_hub()?;
     let dir = home.join("java").join(component);
     let java = dir.join(java_rel());
     let marker = dir.join(MARKER);

     if marker.is_file() && java.is_file() {
          return Ok(java)
     }

     let _guard = LOCK.lock().await;
     
     if marker.is_file() && java.is_file() {
          return Ok(java)
     }

     let client = base();
     let index: Index = get(&client, INDEX).await?;
     let entry = index.machine()?.remove(component).and_then(|bu| bu.into_iter().next()).ok_or_else(|| anyhow!("no component {}", component))?;
     let mani: Manifest = get(&client, &entry.manifest.url).await?;
     let mut jobs: Vec<(PathBuf, Raw, bool)> = Vec::new();
     let mut links: Vec<(PathBuf, String)> = Vec::new();
     for (rel, f) in mani.files {
          let path = safe_mani(&dir, &rel)?;
          match f.kind.as_str() {
               "directory" => fs::create_dir_all(&path).await?,
               "file" => { let raw = f.downloads.ok_or_else(|| anyhow!("{}: no download", rel))?.raw; jobs.push((path, raw, f.executable)); }
               "link" => { if let Some(target) = f.target { links.push((path, target)); }}
               _ => {}
          }
     }

     let results: Vec<Result<()>> = stream::iter(jobs).map(|(p, r, e)| {
          let client = client.clone();
          async move {
               download_with_retry(&client, &r, &p, e).await?;
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
          if hex::encode(h.finalize()).eq_ignore_ascii_case(&raw.sha1) {
               return set_exec(dest, executable).await;
          }
     }
     let id = TEMP_ID.fetch_add(1, Ordering::Relaxed);
     let mut tmp = dest.as_os_str().to_owned();
     tmp.push(format!(".{}.{}.part", std::process::id(), id));
     let tmps = PathBuf::from(tmp);
     let result: Result<()> = async {
          let resp = client.get(&raw.url).send().await?.error_for_status().with_context(|| format!("HTTP error downloading: {}", raw.url))?;
          let mut body = resp.bytes_stream();
          let mut file = OpenOptions::new().write(true).create_new(true).open(&tmps).await?;
          let mut hasher = Sha1::new();

          while let Some(chunk) = body.next().await {
               let chunk = chunk?;
               hasher.update(&chunk);
               file.write_all(&chunk).await?;
          }
          file.flush().await?;
          file.sync_all().await?;
          drop(file);

          let actual = hex::encode(hasher.finalize());
          if !actual.eq_ignore_ascii_case(&raw.sha1) {
               bail!("SHA-1 mismatch for {}: expected {}, got {}", raw.url, raw.sha1, actual);
          }

          replace_download(&tmps, dest).await?;
          set_exec(dest, executable).await?;

          Ok(())
     }.await;
     if result.is_err() {
          let _ = fs::remove_file(&tmps).await;
     }
     result
}

async fn replace_download(temp: &Path, dest: &Path) -> Result<()> {
    #[cfg(not(windows))]
    {
        fs::rename(temp, dest).await?;
    }

    #[cfg(windows)]
    {
        if fs::try_exists(dest).await? {
            let mut backup_name = dest.as_os_str().to_owned();
            backup_name.push(format!(
                ".{}.{}.bak",
                std::process::id(),
                TEMP_ID.fetch_add(1, Ordering::Relaxed)
            ));

            let backup = PathBuf::from(backup_name);

            fs::rename(dest, &backup).await?;

            if let Err(error) = fs::rename(temp, dest).await {
                let _ = fs::rename(&backup, dest).await;
                return Err(error.into());
            }

            let _ = fs::remove_file(backup).await;
        } else {
            fs::rename(temp, dest).await?;
        }
    }

    Ok(())
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