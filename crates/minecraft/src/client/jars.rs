use std::path::{PathBuf};

use clients::base_client;

pub async fn download_client_jar(url: &str, dest_dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
     let client = base_client();
     let dest = dest_dir.join("client.jar");
     if !dest.exists() {
          let bytes = client.get(url).send().await?.bytes().await?;
          std::fs::write(&dest, bytes)?;
     }
     
     Ok(())
}