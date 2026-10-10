use download::{download_deps, save};

#[tauri::command]
pub async fn save_mod(project_id: &str, game: &str, loader: &str) -> Result<(), String> {
     save(project_id, game, loader).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn download_depss(version_id: Option<&str>, project_id: &str, game: &str, loader: &str, inst_name: &str) -> Result<(), String> {
     download_deps(version_id.as_deref(), project_id, game, loader, inst_name).await.map_err(|e| e.to_string())
}