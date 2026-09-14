use std::path::Path;

use crate::client::{Libraries, current_os, applies};

#[allow(unused)]
pub async fn fetch_libraries(libs: &[Libraries], path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>>{
     let os = current_os();
     let mut class = Vec::new();
     
     for lib in libs {
          if !applies(&lib.rules) {
               continue;
          }

          if let Some(artifact) = &lib.downloads.artifact {
               let dest = path.join(&artifact.path);
               if !dest.exists() {
                    std::fs::create_dir_all(dest.parent().unwrap())?;
                    let bys = reqwest::get(&artifact.url).await?.bytes().await?;
                    std::fs::write(&dest, bys)?;
               }
               class.push(dest.to_string_lossy().to_string());
          }
     }
     
     Ok(class)
}