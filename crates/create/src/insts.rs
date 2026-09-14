use std::result::Result;
use std::path::PathBuf;
use std::path::Path;
use std::fs;

use crate::{create_hub::check_dir};
use minecraft::client::client_info::client_url;
use folders::hub_fold::make_hub;

fn unique_name(inst_dir: &Path, desired: String) -> String {
     let can  = inst_dir.join(&desired);
     if !can.exists() {
          return desired;
     }

     let mut counter = 1;
     loop {
          let attempt = format!("{} ({})", desired, counter);
          if !inst_dir.join(&attempt).exists() {
               return attempt
          }
          counter += 1
     }
}

pub async fn setup_inst(app_handle: tauri::AppHandle, inst_name: String, id: String) -> Result<PathBuf, Box<dyn std::error::Error>> {
     let insts_dir = check_dir()?;
     let unique_name = unique_name(&insts_dir, inst_name);
     let inst_dir = insts_dir.join(&unique_name);
     fs::create_dir_all(&inst_dir)?;
     
     let home = make_hub()?.join("versions");
     let vers = home.join(&id).join(&unique_name);

     for sub in ["minecraft"] {
          if !inst_dir.join(sub).exists() {
               fs::create_dir_all(&inst_dir.join(sub))?;
          }
     }
     
     fs::create_dir_all(&vers)?;
     client_url(app_handle, &id, vers, &inst_dir, &unique_name).await.map_err(|e| e.to_string())?;

     Ok(inst_dir)     
}