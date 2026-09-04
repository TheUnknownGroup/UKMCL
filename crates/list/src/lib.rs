use std::io::Result;
use std::fs;
use create::create_hub::check_dir;

pub fn list() -> Result<Vec<String>> {
     let insts_dir = check_dir()?;

     if !insts_dir.is_dir() {
          return Ok(Vec::new());
     }

     let mut names = Vec::new();
     for entry in fs::read_dir(&insts_dir)? {
          let entry = entry?;
          if entry.path().is_dir() {
               if let Some(name) = entry.file_name().to_str() {
                    names.push(name.to_string());
               }
          }
     }
     names.sort();
     Ok(names)
}