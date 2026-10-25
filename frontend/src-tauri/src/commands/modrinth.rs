use store::modrinth::{Hits, modrin_mod, modrin_p, modrin_s};

#[tauri::command]
pub async fn modri(query: &str, loader: &str, game: &str, offset: Option<u64>) -> Result<Hits, String> {
     modrin_mod(query, loader, game, offset.unwrap_or(0)).await
}

#[tauri::command]
pub async fn modri_packs(query: &str, loader: &str, game: &str,  offset: Option<u64>) -> Result<Hits, String> {
     modrin_p(query, loader, game, offset.unwrap_or(0)).await
}

#[tauri::command]
pub async fn modri_shades(query: &str, loader: &str, game: &str, offset: Option<u64>) -> Result<Hits, String> {
     modrin_s(query, loader, game, offset.unwrap_or(0)).await
}