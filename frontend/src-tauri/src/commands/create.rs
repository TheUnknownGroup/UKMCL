use create::insts::setup_inst;

use tauri::AppHandle;
use std::path::PathBuf;

#[tauri::command]
pub async fn create_command(app_handle: AppHandle, inst_name: String, ver: String, loader: String, loader_ver: String) -> Result<PathBuf, String> {
     setup_inst(app_handle, inst_name, ver, loader, loader_ver).await.map_err(|e| e.to_string())
}