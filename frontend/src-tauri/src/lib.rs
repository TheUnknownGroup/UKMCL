// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[cfg_attr(mobile, tauri::mobile_entry_point)]

use std::path::{PathBuf};
use std::fs;

use tauri::AppHandle;
use tauri_plugin_opener::OpenerExt;
use list::list;
use create::{insts::setup_inst, make::make_json, create_hub::check_dir};
use delete::inst_del;
use watch::watch_dir;
use folders::hub_fold::make_hub;
use java::launch;
use windows::{inst::spawn_insts, inst_creation::spawn_inst};

#[tauri::command]
async fn create_command(app_handle: AppHandle, inst_name: String, ver: String, loader: String, loader_ver: String) -> Result<PathBuf, String> {
     setup_inst(app_handle, inst_name, ver, loader, loader_ver).await.map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_command(inst_name: String) -> Result<(), String> {
     inst_del(&inst_name).map_err(|e| e.to_string())
}

#[tauri::command]
async fn launch_command(inst_name: String) -> Result<(), String> {
     launch(&inst_name).map_err(|e| e.to_string())
}

#[tauri::command]
async fn load_versions(loader: String) -> Result<(), String> {
     make_json(&loader).await.map_err(|e| e.to_string())?;
     Ok(())
}
#[tauri::command]
async fn get(loader: String) -> Result<Vec<String>, String> {
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
async fn get_loader(loader: String) -> Result<Vec<String>, String> {
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
async fn spawn_window(app_handle: AppHandle) -> Result<(), String> {
     spawn_inst(&app_handle).await.map_err(|e| e.to_string())?;
     Ok(())
}

#[tauri::command]
async fn spawn_window_2(app_handle: AppHandle, label: String, url: String, name: String) -> Result<(), String> {
     spawn_insts(&app_handle, &label, &url, &name).await.map_err(|e| e.to_string())?;
     Ok(())
}

#[tauri::command]
fn get_command() -> Result<Vec<String>, String> {
   list().map_err(|e| e.to_string())
}

#[tauri::command]
async fn open(app: AppHandle, name: String) -> Result<(), String> {
    let insts = check_dir().map_err(|e| e.to_string())?;
    let inst = insts.join(&name);
    let path = inst.to_string_lossy().to_string();
    app.opener().open_path(path, None::<&str>).map_err(|e| e.to_string())?;
    Ok(())
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
             launch_command,
             load_versions,
             spawn_window,
             spawn_window_2,
             get,
             get_loader,
             open,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
