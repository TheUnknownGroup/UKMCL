use std::io;
use std::result::Result;
use std::path::PathBuf;
use std::path::Path;
use std::fs;

use crate::{create_hub::check_dir};
use minecraft::client::client_info::client_url;

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

fn get_inst_dir(inst_name: String) -> io::Result<PathBuf> {
     let insts_dir = check_dir()?;
     let unique_name = unique_name(&insts_dir, inst_name);
     let inst_dir = insts_dir.join(&unique_name);

     fs::create_dir_all(&inst_dir)?;
     Ok(inst_dir)
}

pub async fn setup_inst(app_handle: tauri::AppHandle, inst_name: String, id: String) -> Result<PathBuf, Box<dyn std::error::Error>> {
     let inst_dir = get_inst_dir(inst_name)?;
     let mine = inst_dir.join("minecraft");
     fs::create_dir_all(&mine)?;
     client_url(app_handle, &id, mine).await.map_err(|e| e.to_string())?;
     
     Ok(inst_dir)     
}