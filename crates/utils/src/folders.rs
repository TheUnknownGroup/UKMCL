use std::{fs, io::{Error, ErrorKind, Result}, path::PathBuf};

fn make_hub() -> Result<PathBuf> {
     let hub = dirs::home_dir().ok_or_else(|| Error::new(ErrorKind::NotFound, "couldn't resolve home directory"))?;
     let base = hub.join(".ukmcl");

     for sub in ["jsons", "assets", "libraries", "versions", "java", "instances"] {
          fs::create_dir_all(base.join(sub))?;
     }
     let assets_root = base.join("assets");
     for assets_sub in ["indexes", "objects"] {
          fs::create_dir_all(assets_root.join(assets_sub))?;
     }
     if !base.is_dir() {
          return Err(Error::new(ErrorKind::NotFound, format!("expected dir not found: {}", base.display())))
     }
     
     Ok(base)
}

pub fn check() -> Result<PathBuf> {
    let home_directory = dirs::home_dir().ok_or_else(|| Error::new(ErrorKind::NotFound, "couldn't resolve home directory"))?.join(".ukmcl");
    if !home_directory.exists() {
        make_hub().map_err(|e| e.to_string()).expect("Home not able to be created");
    }

    Ok(home_directory)
}