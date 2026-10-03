// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[cfg_attr(mobile, tauri::mobile_entry_point)]

mod commands;
use std::sync::Mutex;

use tauri::{State, WindowEvent, Manager};
use watch::watch_dir;
use tauri::async_runtime;
use tokio::time::sleep;
use std::time::Duration;
use commands::{
     rpc::{DiscordRpc, bot}, 
     gets::{get, get_loader, get_java_max, get_java_min, acc_info}, 
     windows::{spawn_window, spawn_window_2, spawn_off}, 
     create::create_command, delete::delete_command, launch::launch_command, load::load_versions, list::get_command, open::open,
     offline::offline_account, ram::ram,
     on_start::write
};
use discord_rich_presence::DiscordIpc;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(DiscordRpc(Mutex::new(None)))
        .on_window_event(|window, event| {
             if window.label() != "UKMCL" {
                  return;
             }
             
             if let WindowEvent::CloseRequested { .. } = event {
                  let state: State<DiscordRpc> = window.state();
                  if let Ok(mut guard) = state.0.lock() {
                       if let Some(mut client) = guard.take() {
                            let _ = client.close();
                       }
                  }
             }
        })
        .setup(|app| {
             watch_dir(app.handle().clone())?;
             let handle = app.handle().clone();
             async_runtime::spawn(async move {
                  sleep(Duration::from_secs(3)).await;
                  match bot(handle) {
                       Ok(_) => println!("Discord RPC: Activity set"),
                       Err(e) => eprintln!("Discord RPC Error: {}", e),
                  }
             });
             Ok(())
        })
        .invoke_handler(tauri::generate_handler![
             get_command,
             create_command,
             delete_command,
             launch_command,
             load_versions,
             spawn_window,
             spawn_window_2,
             spawn_off,
             get,
             get_loader,
             get_java_max, 
             get_java_min,
             acc_info,
             open,
             offline_account,
             ram,
             write,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
