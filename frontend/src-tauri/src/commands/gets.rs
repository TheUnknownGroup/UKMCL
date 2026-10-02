use folders::hub_fold::make_hub;
use std::fs;

use config::main_conf::{AccountConfig, Main};

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