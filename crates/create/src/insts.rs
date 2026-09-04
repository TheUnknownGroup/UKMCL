use std::io::Result;
use std::path::PathBuf;
use std::path::Path;
use std::fs;

use crate::create_hub::check_dir;

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

fn get_inst_dir(inst_name: &str) -> Result<PathBuf> {
     let insts_dir = check_dir()?;
     let unique_name = unique_name(&insts_dir, inst_name);
     let inst_dir = insts_dir.join(&unique_name);

     fs::create_dir_all(&inst_dir)?;
     Ok(inst_dir)
}

pub fn setup_inst(inst_name: &str) -> Result<PathBuf> {
     let inst_dir = get_inst_dir(inst_name)?;

     Ok(inst_dir)     
}