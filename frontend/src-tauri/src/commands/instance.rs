use config::instance::InstanceStruct;
use java::launch;
use maker::setup_instance;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use tauri_plugin_opener::OpenerExt;
use std::{fs, io::ErrorKind, path::Path};
use utils::{folders::make_hub, toasts::{ToastKind, toast}};

#[tauri::command]
pub async fn create_instance(app: AppHandle, name: String, version: String, loader: String, loader_version: String) -> Result<(), String> {
     match setup_instance(&app, name, version, loader, loader_version).await {
          Ok(()) => {
               let _ = app.emit("progress-complete", ());
               let _ = toast(&app, ToastKind::Success, "Successfully created");
               Ok(())
          }
          Err(err) => {
               let message = err.to_string();
               let _ = app.emit("progress-error", &message);
               let _ = toast(&app, ToastKind::Error, &message);
               Err(message)
          }
     }
}

#[tauri::command]
pub async fn launching(name: String, world: Option<&str>, server: Option<&str>) -> Result<(), String> {
     launch(&name, world, server).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_instance(name: &str) -> Result<(), String> {
     let home = make_hub().map_err(|e| e.to_string())?;
     let find_instance = home.join("instances").join(name);
     let cfg = find_instance.join("config.toml");
     let info = InstanceStruct::load(&cfg.to_string_lossy().as_ref()).map_err(|e| e.to_string())?;
     let version = &info.main.minecraft_version;
     let version_directory = home.join("versions").join(version);
     remove_dirs(&version_directory.join(name))?;

     match fs::remove_dir(&version_directory) {
          Ok(()) => {},
          Err(e) if e.kind() == ErrorKind::DirectoryNotEmpty => {},
          Err(e) if e.kind() == ErrorKind::NotFound => {
               let _ = format!("instance not found: {}", name);
          },
          Err(e) => return Err(e.to_string()),
     }

     remove_dirs(&find_instance)?;

     Ok(())
}

fn remove_dirs(path: &Path) -> Result<(), String> {
     match fs::remove_dir_all(path) {
          Ok(()) => Ok(()),
          Err(e) if e.kind() == ErrorKind::NotFound => Ok(()),
          Err(e) => Err(format!("failed to remove {}: {e}", path.display())),
     }
}

#[tauri::command]
pub async fn instance_info(name: &str) -> Result<InstanceStruct, String> {
     let home = make_hub().map_err(|e| e.to_string())?;
     let cfg = home.join("instances").join(name).join("config.toml").to_string_lossy().to_string();
     let info = InstanceStruct::load(&cfg).map_err(|e| e.to_string())?;
     Ok(info)
}

#[tauri::command]
pub async fn open_instance_folder(app: AppHandle, name: String) -> Result<(), String> {
     let home = make_hub().map_err(|e| e.to_string())?;
     let inst = home.join("instances").join(name).to_string_lossy().to_string();
     app.opener().open_path(inst, None::<&str>).map_err(|e| e.to_string())?;
     Ok(())
}

#[derive(Serialize)]
pub struct Save {
     folder: String
}

#[tauri::command]
pub fn list_saves(name: &str) -> Result<Vec<Save>, String> {
     let home = make_hub().map_err(|e| e.to_string())?;
     let conf = home.join("instances").join(name).join("minecraft").join("saves");

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
pub struct ServerDat {
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
pub fn list_servers(name: &str) -> Result<Vec<Server>, String> {
     let home = make_hub().map_err(|e| e.to_string())?;
     let conf = home.join("instances").join(name).join("minecraft").join("servers.dat");

     let Ok(entries) = std::fs::read(&conf) else {
          return Ok(vec![]);
     };
     
     let dat: ServerDat = fastnbt::from_bytes(&entries).map_err(|e| e.to_string())?;

     Ok(dat.servers.into_iter().filter(|s| !s.ip.is_empty()).map(|s| Server {
          name: if s.name.is_empty() { s.ip.clone() } else { s.name },
          address: s.ip,
     }).collect())
}