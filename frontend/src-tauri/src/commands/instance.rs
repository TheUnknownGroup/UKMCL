use maker::setup_instance;
use tauri::AppHandle;

#[tauri::command]
pub async fn create_instance(app: AppHandle, name: String, version: String, loader: String, loader_version: String) -> Result<(), String> {
     setup_instance(app, name, version, loader, loader_version).await.map_err(|e| e.to_string())
}