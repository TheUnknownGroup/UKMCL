use config::writes::build;

#[tauri::command]
pub async fn write() -> Result<(), String>{
     build().await.map_err(|e| e.to_string())?;
     Ok(())
}