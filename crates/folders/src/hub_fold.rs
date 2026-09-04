use std::io::Result;
use std::io::Error;
use std::io::ErrorKind;
use std::fs;
use std::path::PathBuf;

pub fn make_hub() -> Result<PathBuf> {
     let hub = dirs::home_dir().ok_or_else(|| Error::new(ErrorKind::NotFound, "couldn't resolve data dir"))?;
     let base = hub.join(".ukmcl");

     if !base.exists() {
          fs::create_dir_all(&base)?;
     }

     Ok(base)
}