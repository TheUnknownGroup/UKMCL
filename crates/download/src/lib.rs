use std::{collections::HashMap, io::Read, path::Path};
pub mod curseforge;

use create::insts::setup_inst;
use serde::{Serialize, Deserialize};
use clients::base_client;
use tauri::AppHandle;

const API: &str = "https://api.modrinth.com/v2";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Modrinth {
     pub loaders: Vec<String>,
     pub id: String,
     pub name: String,
     pub version_number: String,
     pub files: Vec<File>,
     pub dependencies: Option<Vec<Depends>>
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct File {
     pub url: String,
     pub filename: String,
     pub primary: bool,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Depends {
     pub version_id: Option<String>,
     pub project_id: Option<String>,
     pub file_name: Option<String>,
     pub dependency_type: String,
}

async fn list_version_for_game(project_id: &str, game: &str, loader: &str) -> Result<Vec<Modrinth>, String> {
     let client = base_client();
     let url = format!("{}/project/{}/version?game_versions=[\"{}\"]&loaders=[\"{}\"]", API, project_id, game, loader);
     let resp: Vec<Modrinth> = client.get(&url).send().await.map_err(|e| e.to_string())?.error_for_status().map_err(|e| e.to_string())?.json().await.map_err(|e| e.to_string())?;
     Ok(resp)
}

pub async fn download(url: &str, file: &str, inst_name: &str) -> Result<(), String> {
     let client = base_client();

     let home = folders::hub_fold::make_hub().map_err(|e| e.to_string())?;
     let mods = home.join("instances").join(inst_name).join("minecraft").join("mods");
     if !mods.exists() {
          tokio::fs::create_dir_all(&mods).await.map_err(|e| e.to_string())?;
     }
     let dest = mods.join(file);
     if dest.exists() { return Ok(()); };
     
     let bytes = client.get(url).send().await.map_err(|e| e.to_string())?.error_for_status().map_err(|e| e.to_string())?.bytes().await.map_err(|e| e.to_string())?;
     
     tokio::fs::write(&dest, &bytes).await.map_err(|e| e.to_string())
}

#[derive(Deserialize, Debug)]
pub struct MrPack {
     pub name: String,
     pub files: Vec<Files>,
     pub dependencies: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
pub struct Files {
     pub path: String,
     pub downloads: Vec<String>,
     pub env: Option<Envs>,
}

#[derive(Deserialize, Debug)]
pub struct Envs {
     pub client: String,
}

async fn fetch_to(client: &reqwest::Client, url: &str, dest: &Path) -> Result<(), String> {
     use tokio::io::AsyncWriteExt;
     let mut resp = client.get(url).send().await.map_err(|e| e.to_string())?.error_for_status().map_err(|e| e.to_string())?;
     let part = dest.with_extension("part");
     let mut f = tokio::fs::File::create(&part).await.map_err(|e| e.to_string())?;
     while let Some(chunk) = resp.chunk().await.map_err(|e| e.to_string())? {
          f.write_all(&chunk).await.map_err(|e| e.to_string())?;
     }
     f.flush().await.map_err(|e| e.to_string())?;
     tokio::fs::rename(&part, dest).await.map_err(|e| e.to_string())
}

pub async fn download_p(url: &str, file: &str, app_handle: AppHandle, loader: &str) -> Result<(), String> {
     let client = base_client();
     let home = folders::hub_fold::make_hub().map_err(|e| e.to_string())?;
     let temp_dir = home.join("temp");
     if !temp_dir.exists() { std::fs::create_dir(&temp_dir).map_err(|e| e.to_string())?; }
     let dest = temp_dir.join(file);
     if !dest.exists() { fetch_to(&client, url, &dest).await?; }
     let files = std::fs::File::open(&dest).map_err(|e| e.to_string())?;
     let mut arch = zip::ZipArchive::new(files).map_err(|e| e.to_string())?;
     let mut inde = String::new();
     arch.by_name("modrinth.index.json").map_err(|e| e.to_string())?.read_to_string(&mut inde).map_err(|e| e.to_string())?;
     let index: MrPack = serde_json::from_str(&inde).map_err(|e| e.to_string())?;

     let game = index.dependencies.get("minecraft").ok_or("mani missing mc dep")?;
     let loads = match loader {
          "fabric" => index.dependencies.get("fabric-loader").cloned().ok_or("mani missing loader dep")?,
          "quilt" => index.dependencies.get("quilt-loader").cloned().ok_or("mani missing loader dep")?,
          "neoforge" => index.dependencies.get("neoforge").cloned().ok_or("mani missing loader dep")?,
          other => return Err(other.to_string()),
     };
     let name = format!("{}-{}", index.name, game);
     let inst_dir = setup_inst(app_handle, name, game.to_string(), loader.to_string(), loads).await.map_err(|e| e.to_string())?;
     let inst = inst_dir.join("minecraft");

     for entry in &index.files {
          let mut downloaded = false;         
          if let Some(env) = &entry.env && env.client == "unsupported" {
               continue;
          }

          let dest = inst.join(&entry.path);
          if !dest.starts_with(&inst) {
               return Err(format!("mani tried to write outside: {}", entry.path));
          }
          if let Some(parent) = dest.parent() {
               tokio::fs::create_dir_all(parent).await.map_err(|e| e.to_string())?;
          }
          for url in &entry.downloads {
               if fetch_to(&client, url, &dest).await.is_ok() { downloaded = true;  break; }
               if let Ok(resp) = client.get(url).send().await && resp.status().is_success() && let Ok(b) = resp.bytes().await {
                    tokio::fs::write(&dest, &b).await.map_err(|e| e.to_string())?;
                    downloaded = true;
                    break;
               }
          }
          if !downloaded {
               return Err(format!("failed to download, {}", entry.path));
          }
     }
     overrides(&mut arch, "overrides", &inst)?;

     let defaults = inst.join("config").join("modpack_defaults");
     if defaults.is_dir() {
          copys(&defaults, &inst)?;
     }
     
     let _ = tokio::fs::remove_dir_all(&defaults).await;
     let _ = tokio::fs::remove_dir_all(&temp_dir).await;

     Ok(())
}

pub(crate) fn overrides(arch: &mut zip::ZipArchive<std::fs::File>, prefix: &str, inst: &Path) -> Result<(), String> {
     for i in 0..arch.len() {
          let mut f = arch.by_index(i).map_err(|e| e.to_string())?;
          let Some(name) = f.enclosed_name() else { continue };
          let Ok(rel) = name.strip_prefix(prefix) else { continue };
          if rel.as_os_str().is_empty() { continue; }
          let dest = inst.join(rel);
          if !dest.starts_with(inst) { continue; }
          if f.is_dir() { 
               std::fs::create_dir_all(&dest).map_err(|e| e.to_string())?;
               continue;
          } 
          if let Some(parent) = dest.parent() {
               std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
          }
          let mut buf = std::fs::File::create(&dest).map_err(|e| e.to_string())?;
          std::io::copy(&mut f, &mut buf).map_err(|e| e.to_string())?;
     }
     Ok(())
}

fn copys(fr: &Path, tos: &Path) -> Result<(), String> {
     for entry in std::fs::read_dir(fr).map_err(|e| e.to_string())? {
          let entry = entry.map_err(|e| e.to_string())?;
          let from = entry.path();
          let to = tos.join(entry.file_name());
          let ty = entry.file_type().map_err(|e| e.to_string())?;

          if ty.is_dir() {
               std::fs::create_dir_all(&to).map_err(|e| e.to_string())?;
               copys(&from, &to)?;
          } else {
               std::fs::copy(&from, &to).map_err(|e| e.to_string())?;
          }
     }

     Ok(())
}

pub async fn download_s(url: &str, file: &str, inst_name: &str) -> Result<(), String> {
     let client = base_client();
     
     let home = folders::hub_fold::make_hub().map_err(|e| e.to_string())?;
     let mods = home.join("instances").join(inst_name).join("minecraft").join("shaderpacks");
     if !mods.exists() {
          tokio::fs::create_dir_all(&mods).await.map_err(|e| e.to_string())?;
     }
     let dest = mods.join(file);
     if dest.exists() { return Ok(()); };
     
     let bytes = client.get(url).send().await.map_err(|e| e.to_string())?.error_for_status().map_err(|e| e.to_string())?.bytes().await.map_err(|e| e.to_string())?;
     
     tokio::fs::write(&dest, &bytes).await.map_err(|e| e.to_string())
}

pub async fn download_deps(version_id: Option<&str>, project_id: &str, game: &str, loader: &str, inst_name: &str) -> Result<(), String> {
     let info = list_version_for_game(project_id, game, loader).await?;
     let ver = match version_id {
          Some(id) => info.iter().find(|v| v.id == id),
          None => info.first(),
     }.ok_or_else(|| format!("version not found for {}", project_id))?;
     let file = ver.files.iter().find(|v| v.primary).or_else(|| ver.files.first()).ok_or("no files found")?;
     
     download(&file.url, &file.filename, inst_name).await.map_err(|e| e.to_string())
}

pub async fn save(project_id: &str, game: &str, loader: &str) -> Result<(), String> {
     let home = folders::hub_fold::make_hub().map_err(|e| e.to_string())?;
     let confg = home.join("jsons").join(format!("{}-{}.json", project_id, game));
     let data = list_version_for_game(project_id, game, loader).await?;
     let json = serde_json::to_string_pretty(&data).map_err(|e| e.to_string())?;
     tokio::fs::write(&confg, json).await.map_err(|e| e.to_string())
}

pub async fn load(project_id: &str, game: &str) -> Result<Vec<Modrinth>, String> {
     let home = folders::hub_fold::make_hub().map_err(|e| e.to_string())?;
     let cg = &home.join("jsons").join(format!("{}-{}.json", project_id, game));
     let file = std::fs::read_to_string(cg).map_err(|e| e.to_string())?;
     let info: Vec<Modrinth> = serde_json::from_str(&file).map_err(|e| e.to_string())?;

     Ok(info)
}