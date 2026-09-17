use std::path::{PathBuf};
use tokio::io::AsyncWriteExt;
use futures_util::StreamExt;

use crate::client::{Libraries, applies};

#[allow(unused)]
pub async fn fetch_libraries(libs: &[Libraries], main: &PathBuf) -> Result<Vec<String>, Box<dyn std::error::Error>>{
     let mut class = Vec::new();
     let path = main.join("libraries");
     
     for lib in libs {
          if !applies(&lib.rules) {
               continue;
          }

          if let Some(artifact) = &lib.downloads.artifact {
               let dest = path.join(&artifact.path);
               if !dest.exists() {
                    std::fs::create_dir_all(dest.parent().unwrap())?;
                    let bys = reqwest::get(&artifact.url).await?;
                    let mut stream = bys.bytes_stream();
                    let mut file = tokio::fs::File::create(&dest).await?;
                    while let Some(chunk) = stream.next().await {
                         let chunk = chunk?;
                         file.write_all(&chunk).await?;
                    }
               }
               class.push(dest.to_string_lossy().to_string());
          }
     }
     
     Ok(class)
}