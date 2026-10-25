use clients::base_client;
use serde_json::Value;
use std::{collections::HashSet, error::Error, ffi::OsStr, fs, path::{Path, PathBuf}, process::Stdio};
use tokio::process::Command;
use crate::neoforge::is_legacy;

const MAVEN: &str = "https://maven.neoforged.net/releases/net/neoforged";

fn read_ver_id(jar: &Path) -> Result<String, Box<dyn Error>> {
     let mut zip = zip::ZipArchive::new(fs::File::open(jar)?)?;
     let json: serde_json::Value = serde_json::from_reader(zip.by_name("version.json")?)?;
     Ok(json["id"].as_str().ok_or("no id in installer version.json")?.to_owned())
}

pub async fn install(java: impl AsRef<OsStr>, hub: &Path, mc_id: &str, mc_json: &[u8], client_j: &Path, loader_ver: &str) -> Result<String, Box<dyn Error>> {
     if !is_legacy(loader_ver) {
          let id = format!("neoforge-{loader_ver}");
          if hub.join("versions").join(&id).join(format!("{id}.json")).exists() {
               return Ok(id);
          }
     }  

     let mut staged: Vec<PathBuf> = Vec::new();
     let result = run(java, hub, mc_id, mc_json, client_j, loader_ver, &mut staged).await;

     for f in &staged {
          let _ = fs::remove_file(f);
     }
     let _ = fs::remove_dir_all(hub.join("installers"));
     result
}

async fn run(java: impl AsRef<OsStr>, hub: &Path, mc_id: &str, mc_json: &[u8], client_j: &Path, loader_ver: &str, staged: &mut Vec<PathBuf>) -> Result<String, Box<dyn Error>> {
     let client = base_client();
     
     let vdir = hub.join("versions").join(mc_id);
     if !vdir.exists() {
          fs::create_dir_all(&vdir)?;
     }
     let vjson = vdir.join(format!("{mc_id}.json"));
     if !vjson.exists() {
          fs::write(&vjson, mc_json)?;
          staged.push(vjson);
     }
     let vjar = vdir.join(format!("{mc_id}.jar"));
     if !vjar.exists() {
          fs::copy(client_j, &vjar)?;
          staged.push(vjar);
     }
     let art = if is_legacy(loader_ver) { "forge" } else { "neoforge" };
     let cache = hub.join("installers");
     if !cache.exists() {
          fs::create_dir_all(&cache)?;
     }
     let jar = cache.join(format!("{art}-{loader_ver}-installer.jar"));
     staged.push(jar.clone());
     if !jar.exists() {
          let url = format!("{MAVEN}/{art}/{loader_ver}/{art}-{loader_ver}-installer.jar");
          let bytes = client.get(url).send().await?.error_for_status()?.bytes().await?;
          let part = jar.with_extension("part");
          staged.push(part.clone());
          fs::write(&part, &bytes)?;
          fs::rename(&part, &jar)?;
     }

     let id = read_ver_id(&jar)?;
     let out = hub.join("versions").join(&id).join(format!("{id}.json"));
     if out.exists() {
          return Ok(id);
     }

     let profiles = hub.join("launcher_profiles.json");
     if !profiles.exists() {
          fs::write(&profiles, r#"{"profiles":{}}"#)?;
          staged.push(profiles);
     }

     let stat = Command::new(java).current_dir(&cache).env("JAVA_TOOL_OPTIONS", "-Xmx768M -XX:+UseSerialGC").arg("-jar").arg(&jar).arg("--installClient").arg(hub).stdout(Stdio::null()).stderr(Stdio::null()).status().await?;
     if !stat.success() || !out.exists() {
          let logs = hub.join("logs");
          let _ = fs::create_dir_all(&logs);
          let _ = fs::copy(format!("{}.log", jar.display()), logs.join(format!("{id}-install.log")));
          return Err(format!("loader install failed ({stat}); see logs/{id}-install.txt").into());
     }
     Ok(id)
}

pub struct Profile {
     pub main_class: String,
     pub libraries: Vec<String>,
     pub jvm_args: Vec<String>,
     pub game_args: Vec<String>,
}

pub fn read(hub: &Path, id: &str, client_j: &str) -> Result<Profile, Box<dyn Error>> {
     let path = hub.join("versions").join(id).join(format!("{id}.json"));
     let v: Value = serde_json::from_slice(&fs::read(&path)?)?;
     let lib_dir = hub.join("libraries");
     let sep = if cfg!(windows) { ";" } else { ":" };
     let sub = |s: &str| {
          s.replace("${library_directory}", &lib_dir.to_string_lossy()).replace("${classpath_separator}", sep).replace("${version_name}", client_j)
     };
     let strs = |key: &str| -> Vec<String> {
          v["arguments"][key].as_array().map(|a| a.iter().filter_map(|x| x.as_str()).map(&sub).collect()).unwrap_or_default()
     };
     let mut jvm = Vec::new();
     let mut it = strs("jvm").into_iter();
     while let Some(a) = it.next() {
          if a == "-cp" || a =="-classpath" { it.next(); continue; }
          jvm.push(a);
     }

     let game = strs("game");
     if jvm.iter().chain(&game).any(|a| a.contains("${")) {
          eprintln!("warning: unsubstituted placeholder {jvm:#?} {game:#?}");
     }

     let libs = v["libraries"].as_array().map(|a| {
          a.iter().filter_map(|l| l["downloads"]["artifact"]["path"].as_str()).map(|p| lib_dir.join(p).to_string_lossy().into_owned()).collect()
     }).unwrap_or_default();

     Ok(Profile {
          main_class: v["mainClass"].as_str().ok_or("no mainClass found")?.to_owned(),
          libraries: libs,
          jvm_args: jvm,
          game_args: game,
     })
}

fn lib_key(p: &str) -> String {
     let path = Path::new(p);
     let (Some(ver_dir), Some(stem)) = (path.parent(), path.file_stem().and_then(|s| s.to_str())) else {
          return p.to_owned();
     };
     let Some(art_dir) = ver_dir.parent() else { return p.to_owned() };
     let vers = ver_dir.file_name().and_then(|s| s.to_str()).unwrap_or("");
     let art = art_dir.file_name().and_then(|s| s.to_str()).unwrap_or("");
     let class = stem.strip_prefix(&format!("{art}-{vers}")).unwrap_or("");
     format!("{}|{class}", art_dir.display())
}

pub fn merge(vanilla: &[String], loader: &[String]) -> Vec<String> {
     let replaced: HashSet<String> = loader.iter().map(|p| lib_key(p)).collect();
     loader.iter().cloned().chain(vanilla.iter().filter(|p| !replaced.contains(&lib_key(p))).cloned()).collect()
}