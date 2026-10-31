use std::fs;

use json_minecraft::{ListEntry, file_find, make_version_json};
use utils::folders::make_hub;

#[tauri::command]
pub async fn get_version_list(loader: String) -> Result<Vec<ListEntry>, String> {
     let list = match loader.as_str() {
          "vanilla" | "forge" | "neoforge" => "versions.json",
          "fabric" => "fabric.json",
          "quilt" => "quilt.json",
          _ => return Err(format!("invalid loader: {}", loader)),
     };
     let home = make_hub().map_err(|e| e.to_string())?;
     let all_dir = home.join("jsons").join(list);
     let contents = fs::read_to_string(&all_dir).map_err(|e| e.to_string())?;
     let ids: Vec<ListEntry> = serde_json::from_str(&contents).map_err(|e| e.to_string())?;
     Ok(ids)
}

#[tauri::command]
pub async fn get_loader_list(loader: String) -> Result<Vec<String>, String> {
     let list = file_find(&loader)?;
     let home = make_hub().map_err(|e| e.to_string())?;
     let all_dir = home.join("jsons").join(list);
     let contents = fs::read_to_string(&all_dir).map_err(|e| e.to_string())?;
     let ids: Vec<String> = serde_json::from_str(&contents).map_err(|e| e.to_string())?;
     Ok(ids)
}

#[tauri::command]
pub async fn load_versions(loader: String) -> Result<(), String> {
     make_version_json(&loader).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_command() -> Result<Vec<String>, String> {
     list().map_err(|e| e.to_string())
}

fn list() -> std::io::Result<Vec<String>> {
     let home = make_hub()?;
     let instance_directory = home.join("instances");
     if !instance_directory.is_dir() {
          return Ok(Vec::new());
     }
     let mut names = Vec::new();
     for entry in fs::read_dir(instance_directory)? {
          let entry = entry?;
          if entry.path().is_dir() && let Some(name) = entry.file_name().to_str() {
               names.push(name.to_string());
          }
     }
     names.sort();
     Ok(names)
}