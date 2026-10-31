use tauri::AppHandle;
use utils::windows::{spawn_instance, spawn_instance_creation, spawn_store};

#[tauri::command]
pub async fn spawn_store_window(app: AppHandle, url: &str) -> Result<(), String> {
     spawn_store(&app, url).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn spawn_instance_creation_window(app: AppHandle) -> Result<(), String> {
     spawn_instance_creation(&app).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn spawn_instance_window(app: AppHandle, label: &str, url: &str, name: &str) -> Result<(), String> {
     spawn_instance(&app, label, url, name).await.map_err(|e| e.to_string())
}