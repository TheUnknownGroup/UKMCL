use types::main_dir::make_main;

use std::io;
use std::path::PathBuf;

pub fn add_sub_dirs() -> io::Result<PathBuf> {
     let base = make_main()?;
     for sub in ["assets", "libs", "java"] {
          if !base.join(sub).exists() {
               std::fs::create_dir_all(&base.join(sub))?;
          }  
     }  

     Ok(base)
}