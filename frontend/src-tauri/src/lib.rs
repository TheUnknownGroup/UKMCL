// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
use commands::{
     sub_windows::{spawn_instance_creation_window, spawn_instance_window, spawn_store_window},
     account::{cancel, microsoft_auth, get_auth_info, account_info, write_initial, offline, sign_out},
     java::{get_java_maximum, get_java_minimum},
     version_list::{get_loader_list, get_version_list, load_versions, get_command},
     instance::create_instance,
};
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
             spawn_instance_creation_window, spawn_instance_window, spawn_store_window,
             cancel, microsoft_auth, get_auth_info, account_info, write_initial, offline, sign_out,
             get_java_maximum, get_java_minimum,
             get_version_list, get_loader_list, load_versions, get_command,
             create_instance,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
