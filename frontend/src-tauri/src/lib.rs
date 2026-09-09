// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[cfg_attr(mobile, tauri::mobile_entry_point)]

use std::path::PathBuf;
use std::fs;

use list::list;
use create::insts::setup_inst;
use create::make::make_json;
use delete::inst_del;
use watch::watch_dir;
use folders::hub_fold::make_hub;

#[tauri::command]
async fn create_command(inst_name: String, ver: String) -> Result<PathBuf, String> {
     setup_inst(&inst_name, &ver).await.map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_command(inst_name: String) -> Result<(), String> {
     inst_del(&inst_name).map_err(|e| e.to_string())
}

#[tauri::command]
async fn load_versions() -> Result<(), String> {
     make_json().await.map_err(|e| e.to_string())?;
     Ok(())
}
#[tauri::command]
async fn get() -> Result<Vec<String>, String> {
     let home = make_hub().map_err(|e| e.to_string())?;
     let all_dir = home.join("jsons").join("versions.json");
     // let old_dir = home.join("old.json");
     // let real_dir = home.join("releases.json");
     // let snap_dir = home.join("snapshots.json");
     let conts = fs::read_to_string(&all_dir).map_err(|e| e.to_string())?;
     let ids: Vec<String> = serde_json::from_str(&conts).map_err(|e| e.to_string())?;
     Ok(ids)
}

#[tauri::command]
fn get_command() -> Result<Vec<String>, String> {
   list().map_err(|e| e.to_string())
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
             watch_dir(app.handle().clone())?;
             Ok(())
        })
        .invoke_handler(tauri::generate_handler![
             get_command,
             create_command,
             delete_command,
             load_versions,
             get,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
