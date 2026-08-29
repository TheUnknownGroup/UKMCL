// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[cfg_attr(mobile, tauri::mobile_entry_point)]

use std::path::PathBuf;
use std::fs;
use acore::dir::main_and_inst::setup_mani;
use acore::dir::list_dir::list;
use acore::dir::watch::watch_dir;
use acore::dir::delete_dir::delete_inst;
use middle_man::load_vers::load_ids;
use types::main_dir::make_main;

#[tauri::command]
fn delete_command(instance_name: String) -> Result<(), String> {
    delete_inst(&instance_name).map_err(|e| e.to_string())
}

#[tauri::command]
async fn load_versions() -> Result<(), String> {
    load_ids().await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn get_ver_id() -> Result<Vec<String>, String> {
    let home = make_main().map_err(|e| e.to_string())?;
    let json = home.join("versions.json");
    let conts = fs::read_to_string(&json).map_err(|e| e.to_string())?;
    let ids: Vec<String> = serde_json::from_str(&conts).map_err(|e| e.to_string())?;
    Ok(ids)
}

#[tauri::command]
async fn create_command(instance_name: String, version: String) -> Result<PathBuf, String> {
    setup_mani(&instance_name, &version)
         .await
         .map_err(|e| e.to_string())
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
            create_command,
            get_command,
            delete_command,
            load_versions,
            get_ver_id,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
