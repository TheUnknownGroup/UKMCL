// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
use std::{sync::Mutex, time::Duration};

use auth::start_refresh;
use commands::{
     sub_windows::{spawn_instance_creation_window, spawn_instance_window, spawn_store_window, spawn_progress_window},
     account::{cancel, microsoft_auth, get_auth_info, account_info, write_initial, offline, sign_out},
     java::{get_java_maximum, get_java_minimum},
     version_list::{get_loader_list, get_version_list, load_versions, get_command},
     instance::{create_instance, launching, delete_instance, open_instance_folder, list_saves, list_servers, instance_info},
};
use discord_rich_presence::DiscordIpc;
use tauri::{Manager, State, WindowEvent, async_runtime};
use tokio::time::sleep;
use crate::commands::{rpc::{DiscordRpc, bot}, watch::watch_directory};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
             spawn_instance_creation_window, spawn_instance_window, spawn_store_window, spawn_progress_window,
             cancel, microsoft_auth, get_auth_info, account_info, write_initial, offline, sign_out,
             get_java_maximum, get_java_minimum,
             get_version_list, get_loader_list, load_versions, get_command,
             create_instance, launching, delete_instance, open_instance_folder, list_saves, list_servers, instance_info,
        ])
        .manage(DiscordRpc(Mutex::new(None)))
        .on_window_event(|w, e| {
             if w.label() != "main" {
                  return;
             }
             if let WindowEvent::CloseRequested { .. } = e {
                  let state: State<DiscordRpc> = w.state();
                  if let Ok(mut guard) = state.0.lock() && let Some(mut client) = guard.take() {
                       let _ = client.close();
                  }
             }
        })
        .setup(|a| {
             watch_directory(a.handle().clone())?;
             let handle = a.handle().clone();
             async_runtime::spawn(async move {
                  sleep(Duration::from_secs(3)).await;
                  match bot(handle.clone()) {
                       Ok(_) => println!("Discord RPC: Activity set"),
                       Err(e) => eprintln!("Discord RPC Error: {}", e),
                  }
                  start_refresh(handle.clone()).await.map_err(|e| e.to_string())
             });
             Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
