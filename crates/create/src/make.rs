use std::{path::PathBuf};

use minecraft::req::{fabric, fabric_load, fetch, quilt, quilt_load};
use folders::hub_fold::make_hub;

pub async fn make_json(loader: &str) -> Result<(), Box<dyn std::error::Error>> {
     let home = make_hub()?.join("jsons");
     let all: Vec<String> = match loader {
          "vanilla" => {
               let mani = fetch().await?;
               mani.versions.iter()
                    .map(|v| v.id.clone()).collect()
          },
          "fabric" => {
               let mani = fabric().await?;
               mani.into_iter()
                    .map(|g| g.version)
                    .collect()
          },
          "quilt" => {
               let mani = quilt().await?;
               mani.into_iter()   
                    .map(|g| g.version)
                    .collect()
          },
          other => return Err(format!("unknown loader: {}", other).into()),
     };

     let loaders: Option<Vec<String>> = match loader {
          "vanilla" => None,
          "fabric" => Some(fabric_load().await?
               .into_iter().map(|g| g.version).collect()),
          "quilt" => Some(quilt_load().await?
               .into_iter().map(|g| g.version).collect()),
          other => return Err(format!("Unknown loader found: {}", other).into()),
     };
     
     let file = if loader == "vanilla" {
         "versions.json".to_string()
     } else {
         format!("{}.json", loader)
     };  
     write(&home, &file, &all)?;
     
     if let Some(list) = loaders {
          write_loader(home, file2(loader)?, &list)?;
     }
     Ok(())
}

pub fn file2(loader: &str) -> Result<&'static str, String> {
     match loader {
          "vanilla" => Err("no loader to be found".into()),
          "fabric" => Ok("fabric_load.json"),
          "quilt" => Ok("quilt_load.json"),
          other => Err(format!("unknown loader: {}", other)),
     }
}

pub fn write(home: &PathBuf, file: &str, bys: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
     let json_dir = home.join(file);
     
     let file = serde_json::to_string_pretty(bys)?;

     if !json_dir.exists() {
          std::fs::write(&json_dir, file)?;
     } else {
          let raw = std::fs::read_to_string(&json_dir)?;
          let existing: Option<Vec<String>> = serde_json::from_str(&raw).ok();
          if existing.as_ref() != Some(bys) {
               let file = serde_json::to_string_pretty(bys)?;
               std::fs::write(&json_dir, file)?;
          }
     }
     Ok(())
}

pub fn write_loader(home: PathBuf, file: &str, bys: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
     let json_dir = home.join(file);
     
     let file = serde_json::to_string_pretty(bys)?;

     if !json_dir.exists() {
          std::fs::write(&json_dir, file)?;
     } else {
          let raw = std::fs::read_to_string(&json_dir)?;
          let existing: Option<Vec<String>> = serde_json::from_str(&raw).ok();
          if existing.as_ref() != Some(bys) {
               let file = serde_json::to_string_pretty(bys)?;
               std::fs::write(&json_dir, file)?;
          }
     }
     Ok(())
}