// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[cfg_attr(mobile, tauri::mobile_entry_point)]

use std::path::PathBuf;
use acore::dir::main_and_inst::setup_instance;
use acore::dir::list_dir::list;
use acore::dir::watch::watch_dir;
use acore::dir::delete_dir::delete_inst;

#[tauri::command]
fn delete_command(instance_name: String) -> Result<(), String> {
    delete_inst(&instance_name).map_err(|e| e.to_string())
}

#[tauri::command]
fn create_command(instance_name: String) -> Result<PathBuf, String> {
    setup_instance(&instance_name).map_err(|e| e.to_string())
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
