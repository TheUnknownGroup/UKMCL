use std::path::{PathBuf};

pub async fn download_client_jar(url: &str, dest_dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
     let dest = dest_dir.join("client.jar");
     if !dest.exists() {
          let bytes = reqwest::get(url).await?.bytes().await?;
          std::fs::write(&dest, bytes)?;
     }
     
     Ok(())
}