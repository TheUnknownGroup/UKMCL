// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[cfg_attr(mobile, tauri::mobile_entry_point)]

use std::path::PathBuf;

use list::list;
use create::insts::setup_inst;

#[tauri::command]
fn create_command(inst_name: String) -> Result<PathBuf, String> {
     setup_inst(&inst_name).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_command() -> Result<Vec<String>, String> {
   list().map_err(|e| e.to_string())
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
             get_command,
             create_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
