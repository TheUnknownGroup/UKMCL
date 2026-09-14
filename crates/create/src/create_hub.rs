use std::io::Result;
use std::io::Error;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::fs;
use folders::hub_fold::make_hub;

pub fn check_dir() -> Result<PathBuf> {
     let root = make_hub()?;
     let insts_fold = root.join("instances");

     for sub in ["jsons", "assets", "libraries", "versions"] {
          if !root.join(sub).exists() {
               fs::create_dir_all(&root.join(sub))?;
          }
     }
     
     let assets_root = root.join("assets");
     for assets_sub in ["indexes", "objects"] {
          if !assets_root.join(assets_sub).exists() {
               fs::create_dir_all(&assets_root.join(assets_sub))?;
          }
     }

     if !root.exists() {
          fs::create_dir_all(&insts_fold)?;
     } else if !insts_fold.exists() {
          fs::create_dir_all(&insts_fold)?;
     } else if !root.is_dir() {
          return Err(Error::new(
               ErrorKind::NotFound, 
               format!("expected dir not found: {}", root.display()),
          ))
     }

     Ok(insts_fold)
}