use create::create_hub::check_dir;
use std::io::Result;
use std::io::Error;
use std::io::ErrorKind;
use std::fs;

pub fn inst_del(inst_name: &str) -> Result<()> {
     let insts_dir = check_dir()?;
     let inst_dir = insts_dir.join(inst_name);
     if !inst_dir.is_dir() {
          return Err(Error::new(ErrorKind::NotFound,
               format!("inst not found: {}", inst_name),
          ));
     }
     fs::remove_dir_all(&inst_dir)?;
     
     Ok(())
}