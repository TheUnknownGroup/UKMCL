use tauri::AppHandle;

use tauri_plugin_opener::OpenerExt;
use create::{create_hub::check_dir};

#[tauri::command]
pub async fn open(app: AppHandle, name: String) -> Result<(), String> {
    let insts = check_dir().map_err(|e| e.to_string())?;
    let inst = insts.join(&name);
    let path = inst.to_string_lossy().to_string();
    app.opener().open_path(path, None::<&str>).map_err(|e| e.to_string())?;
    Ok(())
}