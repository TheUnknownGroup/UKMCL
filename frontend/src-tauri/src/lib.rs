// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[cfg_attr(mobile, tauri::mobile_entry_point)]
mod commands;
use std::sync::Mutex;

use auth::refresh::refresh_start;
use commands::{
    cancel::cancel,
    create::create_command,
    delete::{delete_command, delete_mod_json},
    gets::{
        acc_info, auths, downloads, get, get_java_max, get_java_min, get_loader, inst_info,
        list_mod_info, list_saves, list_servs,
    },
    launch::launch_command,
    list::get_command,
    load::load_versions,
    micro::microsoft_auth,
    curseforge::{curse, cf_install},
    modrinth::{modri, modri_packs, modri_shades},
    offline::offline_account,
    on_start::write,
    open::open,
    ram::ram,
    rpc::{DiscordRpc, bot},
    save_mod::{save_mod, download_depss},
    windows::{spawn_mods, spawn_modss, spawn_off, spawn_window, spawn_window_2},
};
use discord_rich_presence::DiscordIpc;
use std::time::Duration;
use tauri::async_runtime;
use tauri::{Manager, State, WindowEvent};
use tokio::time::sleep;
use watch::watch_dir;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(DiscordRpc(Mutex::new(None)))
        .on_window_event(|window, event| {
            if window.label() != "main" {
                return;
            }

            if let WindowEvent::CloseRequested { .. } = event {
                let state: State<DiscordRpc> = window.state();
                if let Ok(mut guard) = state.0.lock() && let Some(mut client) = guard.take() {
                    let _ = client.close();
                }
            }
        })
        .setup(|app| {
            watch_dir(app.handle().clone())?;
            let handle = app.handle().clone();
            async_runtime::spawn(async move {
                sleep(Duration::from_secs(3)).await;
                match bot(handle.clone()) {
                    Ok(_) => println!("Discord RPC: Activity set"),
                    Err(e) => eprintln!("Discord RPC Error: {}", e),
                }
                refresh_start().await.map_err(|e| e.to_string())
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_command,
            create_command,
            delete_command, delete_mod_json,
            launch_command,
            load_versions,
            spawn_window, spawn_window_2, spawn_off, spawn_mods, spawn_modss,
            get, get_loader, get_java_max, get_java_min, acc_info, auths, inst_info, list_mod_info, downloads, list_saves, list_servs,
            open,
            offline_account, microsoft_auth,
            ram,
            write,
            cancel,
            curse, cf_install,
            modri, modri_packs, modri_shades,
            save_mod, download_depss,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
