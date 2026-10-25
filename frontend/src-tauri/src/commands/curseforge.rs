use download::curseforge::{Skipped, install_pack, install_project};
use store::{curseforge::cf_search, modrinth::Hits};
use tauri::AppHandle;

#[tauri::command]
pub async fn curse(kind: &str, query: &str, loader: &str, game: &str, offset: Option<u64>) -> Result<Hits, String> {
     cf_search(kind, query, loader, game, offset.unwrap_or(0)).await
}

#[tauri::command]
pub async fn cf_install(app: AppHandle, kind: &str, project_id: &str, game: &str, loader: &str, inst_name: &str) -> Result<Vec<Skipped>, String> {
     let id: u64 = project_id.parse().map_err(|_| format!("bad project id: {project_id}"))?;
     match kind {
          "modpacks" => install_pack(app, id, game, loader).await,
          _ => install_project(kind, id, game, loader, inst_name).await,
     }
}