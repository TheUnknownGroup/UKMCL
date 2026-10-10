use folders::hub_fold::make_hub;
use std::fs;

use config::{main_conf::{AccountConfig, Main}, msa::AuthMSA};
use download::{Modrinth, download, load};
use serde::{Serialize, Deserialize};

#[tauri::command]
pub async fn get(loader: String) -> Result<Vec<String>, String> {
     let file = match loader.as_str() {
          "vanilla" => "versions.json",
          "fabric" => "fabric.json",
          "quilt" => "quilt.json",
          other => return Err(format!("unknown loader: {}", other)),
     };
     let home = make_hub().map_err(|e| e.to_string())?;
     let all_dir = home.join("jsons").join(file);
     
     let conts = fs::read_to_string(&all_dir).map_err(|e| e.to_string())?;
     let ids: Vec<String> = serde_json::from_str(&conts).map_err(|e| e.to_string())?;
     Ok(ids)
}

#[tauri::command]
pub async fn get_loader(loader: String) -> Result<Vec<String>, String> {
     let file = match loader.as_str() {
          "vanilla" => return Err("no vanilla loader found".into()),
          "fabric" => "fabric_load.json",
          "quilt" => "quilt_load.json",
          other => return Err(format!("unknown loader: {}", other)),
     };
     let home = make_hub().map_err(|e| e.to_string())?;
     let all_dir = home.join("jsons").join(file);
     
     let conts = fs::read_to_string(&all_dir).map_err(|e| e.to_string())?;
     let ids: Vec<String> = serde_json::from_str(&conts).map_err(|e| e.to_string())?;
     Ok(ids)
}

#[tauri::command]
pub async fn get_java_min() -> Result<u32, String> {
     let home = make_hub().map_err(|e| e.to_string())?;
     let conf = home.join("config.toml");

     let min = Main::load(&conf.to_string_lossy().to_string()).map_err(|e| e.to_string())?;
     
     Ok(min.java.min)
}

#[tauri::command]
pub async fn get_java_max() -> Result<u32, String> {
     let home = make_hub().map_err(|e| e.to_string())?;
     let conf = home.join("config.toml");

     let max = Main::load(&conf.to_string_lossy().to_string()).map_err(|e| e.to_string())?;
     
     Ok(max.java.max)
}

#[tauri::command]
pub async fn acc_info() -> Result<AccountConfig, String> {
     let home = make_hub().map_err(|e| e.to_string())?;
     let conf = home.join("config.toml");
     let acc = Main::load(&conf.to_string_lossy().to_string()).map_err(|e| e.to_string())?;

     Ok(acc.account)
}

#[tauri::command]
pub async fn auths() -> Result<AuthMSA, String> {
     let home = make_hub().map_err(|e| e.to_string())?;
     let conf = home.join("msa.toml").to_string_lossy().to_string();
     let auth = AuthMSA::load(&conf).map_err(|e| e.to_string())?;

     Ok(auth)
}

#[tauri::command]
pub async fn inst_info(name: &str) -> Result<config::Main, String> {
     let home = make_hub().map_err(|e| e.to_string())?;
     let conf = home.join("instances").join(name).join("config.toml").to_string_lossy().to_string();
     let info = config::Main::load(&conf).map_err(|e| e.to_string())?;

     Ok(info)
} 

#[tauri::command]
pub async fn list_mod_info(project_id: &str, game_ver: &str) -> Result<Vec<Modrinth>, String> {
     load(project_id, game_ver).await
}

#[tauri::command]
pub async fn downloads(url: &str, file: &str, inst_name: &str) -> Result<(), String> {
     download(url, file, inst_name).await
}

#[derive(Serialize)]
pub struct Save {
     folder: String
}

#[tauri::command]
pub fn list_saves(inst_name: &str) -> Result<Vec<Save>, String> {
     let home = make_hub().map_err(|e| e.to_string())?;
     let conf = home.join("instances").join(inst_name).join("minecraft").join("saves");

     let Ok(entries) = std::fs::read_dir(&conf) else {
          return Ok(vec![]);
     };
     
     let mut saves = Vec::new();
     for entry in entries.flatten() {
          let Ok(folder) = entry.file_name().into_string() else { continue };
          saves.push(Save { folder });
     }

     Ok(saves)
}

#[derive(Deserialize)]
pub struct ServDat {
     #[serde(default)]
     pub servers: Vec<ServerEntry>,
}

#[derive(Deserialize)]
pub struct ServerEntry {
     #[serde(default)]
     pub name: String,
     #[serde(default)]
     pub ip: String,
}

#[derive(Serialize)]
pub struct Server {
     name: String,
     address: String,
}

#[tauri::command]
pub fn list_servs(inst_name: &str) -> Result<Vec<Server>, String> {
     let home = make_hub().map_err(|e| e.to_string())?;
     let conf = home.join("instances").join(inst_name).join("minecraft").join("servers.dat");

     let Ok(entries) = std::fs::read(&conf) else {
          return Ok(vec![]);
     };
     
     let dat: ServDat = fastnbt::from_bytes(&entries).map_err(|e| e.to_string())?;

     Ok(dat.servers.into_iter().filter(|s| !s.ip.is_empty()).map(|s| Server {
          name: if s.name.is_empty() { s.ip.clone() } else { s.name },
          address: s.ip,
     }).collect())
}