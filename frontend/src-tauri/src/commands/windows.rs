use tauri::AppHandle;

use windows::{inst::spawn_insts, inst_creation::spawn_inst, offline::spawn_offs};

#[tauri::command]
pub async fn spawn_window(app_handle: AppHandle) -> Result<(), String> {
     spawn_inst(&app_handle).await.map_err(|e| e.to_string())?;
     Ok(())
}

#[tauri::command]
pub async fn spawn_window_2(app_handle: AppHandle, label: String, url: String, name: String) -> Result<(), String> {
     spawn_insts(&app_handle, &label, &url, &name).await.map_err(|e| e.to_string())?;
     Ok(())
}

#[tauri::command]
pub async fn spawn_off(app_handle: AppHandle) -> Result<(), String> {
     spawn_offs(&app_handle).await.map_err(|e| e.to_string())?;
     Ok(())
}