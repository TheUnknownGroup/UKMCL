use std::{collections::HashSet, ffi::OsStr, fs::{self, File}, io::Write, path::{Component, Path, PathBuf}};
use anyhow::{Context, Result, bail};
use serde_json::Value;
use tauri::AppHandle;
use tokio::{process::Command};
use utils::{client::base, progress::emit_progress};
use zip::ZipArchive;

use crate::request::neoforge::is_legacy;

const MAVEN: &str = "https://maven.neoforged.net/releases/net/neoforged";

fn validate(id: &str) -> Result<()> {
     if id.is_empty() || id.len() > 128 || !id.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-' | b'+')) {
          bail!("installer returned an invalid version id: {id:?}");
     }
     Ok(())
}

fn read_ver(installer: &Path) -> Result<String> {
     let file = File::open(installer).with_context(|| format!("opening Neoforge installer {}", installer.display()))?;
     let mut archive = ZipArchive::new(file).context("reading Neoforge installer archive")?;
     let mut version_json = archive.by_name("version.json").context("Neoforge installer does not contain version.json")?;
     let value: Value = serde_json::from_reader(&mut version_json).context("parsing installer version.json")?;
     let id = value["id"].as_str().context("isntaller version.json has no string id")?;
     validate(id)?;
     Ok(id.to_owned())
}

fn safe_library(value: &str) -> Result<PathBuf> {
     if value.is_empty() || value.contains('\\') || value.contains(':') || value.contains('\0') || value.chars().any(char::is_control) {
          bail!("invalid Neoforge library path: {value:?}");
     }
     let path = Path::new(value);
     if path.is_absolute() || path.components().any(|p| !matches!(p, Component::Normal(_))) || value.split('/').any(|p| p.is_empty() || p == "." || p == "..") {
          bail!("unsafe Neoforge library page: {value:?}");
     }
     Ok(path.to_path_buf())
}

pub async fn install(app: &AppHandle, java: impl AsRef<OsStr>, hub: &Path, mc_id: &str, mc_json: &[u8], mc_jar: &Path, version: &str) -> Result<String> {
     let cache = hub.join("installers");
     fs::create_dir_all(&cache)?;
     let art = if is_legacy(version) { "forge" } else { "neoforge" };
     let installer = cache.join(format!("{art}-{version}-installer.jar"));
     let modern_id = format!("neoforge-{version}");
     if !is_legacy(version) && installed_profile(hub, &modern_id, "client") {
          return Ok(modern_id);
     }
     if !installer.is_file() || read_ver(&installer).is_err() {
          download_installer(app, &installer, art, version).await?;
     }
     let id = read_ver(&installer)?;
     if installed_profile(hub, &id, "client") {
          return Ok(id);
     }

     let van_dir = hub.join("versions").join(mc_id);
     fs::create_dir_all(&van_dir)?;
     let van_json = van_dir.join(format!("{mc_id}.json"));
     let van_jar = van_dir.join(format!("{mc_id}.jar"));
     let mut staged = Vec::new();
     if !van_json.exists() {
          write_new(&van_json, mc_json)?;
          staged.push(van_json.clone());
     }

     if !van_jar.exists() {
          fs::copy(mc_jar, &van_jar).with_context(|| {
               format!("staging Minecraft client jar to {}", van_jar.display())
          })?;
          staged.push(van_jar.clone());
     }

     let profiles = hub.join("launcher_profiles.json");
     if !profiles.exists() {
          write_new(&profiles, br#"{"profiles":{}}"#)?;
          staged.push(profiles);
     }

     emit_progress(app, "neoforge", 0, 1, "Running Neoforge installer", 0, 1)?;
     let output = match Command::new(java).current_dir(&cache).env("JAVA_TOOL_OPTIONS", "-Xmx768M -XX:+UseSerialGC").arg("-jar").arg(&installer).arg("--installClient").arg(hub).output().await {
          Ok(output) => output,
          Err(err) => {
               for path in &staged {
                    let _ = fs::remove_file(path);
               }
               return Err(err).context("starting Neoforge installer");
          }
     };
     let profile_result = read(hub, &id, &mc_jar.to_string_lossy().to_owned());
     let log_dir = hub.join("logs");
     let log_path = log_dir.join(format!("{id}-install.log"));
     if !output.status.success() || profile_result.is_err() {
          let _ = fs::create_dir_all(&log_dir);
          let mut log = Vec::new();
          log.extend_from_slice(b"--- stdout ---\n");
          log.extend_from_slice(&output.stdout);
          log.extend_from_slice(b"\n--- stderr ---\n");
          log.extend_from_slice(&output.stderr);
          if let Err(err) = &profile_result {
               log.extend_from_slice(format!("\n--- profile validation ---\n{err:#}\n").as_bytes());
          }
          let _ = fs::write(&log_path, log);
          for path in &staged {
               let _ = fs::remove_file(path);
          }
          bail!("Neoforge installer failed ({}) or produced an incomplete profile. See {}", output.status, log_path.display());
     }

     for path in &staged {
          let _ = fs::remove_file(path);
     }
     
     emit_progress(app, "neoforge", 1, 1, "Neoforge", 1, 1)?;
     let _ = fs::remove_dir_all(hub.join("installers"));
     Ok(id)
}

async fn download_installer(app: &AppHandle, dest: &Path, art: &str, version: &str) -> Result<()> {
     let url = format!("{MAVEN}/{art}/{version}/{art}-{version}-installer.jar");
     let resp = base().get(&url).send().await?.error_for_status().with_context(|| format!("downloading Neoforge installer from {url}"))?;
     let total = resp.content_length().unwrap_or(0);
     emit_progress(app, "neoforge", 0, total.max(1), "Downloading Neoforge installer", 0, 2)?;
     let mut response = resp;
     let mut downloaded = 0_u64;
     let mut temp = dest.as_os_str().to_owned();
     temp.push(format!(".{}.part", std::process::id()));
     let temp = PathBuf::from(temp);
     let result: Result<()> = async {
          let mut file = tokio::fs::File::create(&temp).await?;
          use tokio::io::AsyncWriteExt;
          while let Some(chunk) = response.chunk().await? {
               downloaded += chunk.len() as u64;
               file.write_all(&chunk).await?;
               emit_progress(app, "neoforge", downloaded, total.max(downloaded).max(1), "Downloading Neoforge installer", 0, 2)?;
          }
          file.flush().await?;
          file.sync_all().await?;
          Ok(())
     }.await;

     if let Err(err) = result {
          let _ = tokio::fs::remove_file(&temp).await;
          return Err(err);
     }

     if let Err(err) = read_ver(&temp) {
          let _ = tokio::fs::remove_file(&temp).await;
          return Err(err).context("downloaded Neoforge installer is not a valid installer jar");
     }

     #[cfg(windows)]
     if dest.exists() {
          fs::remove_file(dest)?;
     }
     fs::rename(&temp, dest)?;
     emit_progress(app, "neoforge", downloaded.max(1), downloaded.max(1), "Neoforge install downloaded", 1, 2)?;
     Ok(())
}

fn write_new(path: &Path, bytes: &[u8]) -> Result<()> {
     let mut file = fs::OpenOptions::new().write(true).create_new(true).open(path)?;
     file.write_all(bytes)?;
     file.sync_all()?;
     Ok(())
}

fn installed_profile(hub: &Path, id: &str, client_j: &str) -> bool {
     read(hub, id, &client_j).is_ok()
}

#[derive(Debug)]
pub struct Profile {
     pub main_class: String,
     pub libraries: Vec<String>,
     pub jvm_args: Vec<String>,
     pub game_args: Vec<String>
}

pub fn read(hub: &Path, id: &str, client_jar: &str) -> Result<Profile> {
     validate(id)?;
     let path = hub.join("versions").join(id).join(format!("{id}.json"));
     let value: Value = serde_json::from_slice(&fs::read(&path).with_context(|| format!("reading Neoforge profile: {}", path.display()))?)?;
     let main_class = value["mainClass"].as_str().context("Neoforge profile has no main Class")?.to_owned();
     let sep = if cfg!(windows) { ";" } else { ":" };
     let lib_root = hub.join("libraries");
     let sub = |s: &str| {
          s.replace("${library_directory}", &lib_root.to_string_lossy()).replace("${classpath_separator}", sep).replace("${version_name}", client_jar)
     };

     let strs = |key: &str| -> Vec<String> {
          value["arguments"][key].as_array().map(|a| a.iter().filter_map(|x| x.as_str()).map(&sub).collect()).unwrap_or_default()
     };
     let mut jvm = Vec::new();
     let mut it = strs("jvm").into_iter();
     while let Some(a) = it.next() {
          if a == "-cp" || a == "-classpath" { it.next(); continue; }
          jvm.push(a);
     }

     let game = strs("game");
     if jvm.iter().chain(&game).any(|a| a.contains("${")) {
          eprintln!("warning: unsubstituted: {jvm:#?} {game:#?}");
     }
     
     let mut libraries = Vec::new();
     if let Some(entr) = value["libraries"].as_array() {
          for entry in entr {
               let Some(relative) = entry["downloads"]["artifact"]["path"].as_str() else {
                    continue;
               };
               let relative = safe_library(relative)?;
               let path = lib_root.join(relative);
               if !path.is_file() {
                    bail!("Neoforge library is missing {}", path.display());
               }
               libraries.push(path.to_string_lossy().into_owned());
          }
     }
     Ok(Profile { main_class, libraries, jvm_args: jvm, game_args: game})
}

fn library_key(val: &str) -> String {
     let path = Path::new(val);
     let (Some(version_dir), Some(stem)) = (
          path.parent(), path.file_stem().and_then(|p| p.to_str()),
     ) else {
          return val.to_owned();
     };
     let Some(artifact_dir) = version_dir.parent() else {
          return val.to_owned()
     };
     let version = version_dir.file_name().and_then(|p| p.to_str()).unwrap_or("");
     let art = artifact_dir.file_name().and_then(|p| p.to_str()).unwrap_or("");
     let class = stem.strip_prefix(&format!("{art}-{version}")).unwrap_or("");
     format!("{}|{class}", artifact_dir.display())
}

pub fn merge(vanilla: &[String], loader: &[String]) -> Vec<String> {
     let replaced: HashSet<String> = loader.iter().map(|p| library_key(p)).collect();
     loader.iter().cloned().chain(vanilla.iter().filter(|p| !replaced.contains(&library_key(p))).cloned()).collect()
}