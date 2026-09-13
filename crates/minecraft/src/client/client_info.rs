use std::error::Error;
use std::path::PathBuf;

use crate::id::fetches;
use crate::client::fetchs;

pub async fn client_url(app_handle: tauri::AppHandle, id: &str, inst_dir: PathBuf) -> Result<(), Box<dyn Error>> {
    let mani = fetches(id).await?;
    let id_final = &mani.url;

    fetchs(app_handle, id_final, inst_dir).await?;
    
    Ok(())
}
