use std::io;
use std::result::Result;
use std::path::PathBuf;
use std::path::Path;
use std::fs;

use crate::create_hub::check_dir;
use minecraft::client::client_info::client_url;
use folders::hub_fold::make_hub;

fn unique_name(inst_dir: &Path, desired: &str) -> String {
     let can  = inst_dir.join(desired);
     if !can.exists() {
          return desired.to_string();
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

fn get_inst_dir(inst_name: &str) -> io::Result<PathBuf> {
     let insts_dir = check_dir()?;
     let unique_name = unique_name(&insts_dir, inst_name);
     let inst_dir = insts_dir.join(&unique_name);

     fs::create_dir_all(&inst_dir)?;
     Ok(inst_dir)
}

pub async fn setup_inst(inst_name: &str, id: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
     let inst_dir = get_inst_dir(inst_name)?;

     let home = make_hub()?;
     let ver_fold = home.join("versions");
     fs::create_dir_all(&ver_fold)?;
     client_url(&id.to_string(), &ver_fold).await.map_err(|e| e.to_string())?;
     

     Ok(inst_dir)     
}