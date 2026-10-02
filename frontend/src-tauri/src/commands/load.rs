use create::make::make_json;

#[tauri::command]
pub async fn load_versions(loader: String) -> Result<(), String> {
     make_json(&loader).await.map_err(|e| e.to_string())?;
     Ok(())
}