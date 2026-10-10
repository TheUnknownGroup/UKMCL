use mods::{ModDisplay, search_mod};

#[tauri::command]
pub async fn search(query: &str, loader: &str, game: &str, offset: Option<u64>) -> Result<Vec<ModDisplay>, String> {
     search_mod(query, loader, game, offset.unwrap_or(0)).await
}