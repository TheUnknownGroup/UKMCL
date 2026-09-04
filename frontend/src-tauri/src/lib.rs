// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[cfg_attr(mobile, tauri::mobile_entry_point)]

use std::path::PathBuf;

use list::list;
use create::insts::setup_inst;
use delete::inst_del;
use watch::watch_dir;

#[tauri::command]
fn create_command(inst_name: String) -> Result<PathBuf, String> {
     setup_inst(&inst_name).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_command(inst_name: String) -> Result<(), String> {
     inst_del(&inst_name).map_err(|e| e.to_string())
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
