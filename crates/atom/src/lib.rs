use std::{error::Error, fs, io, path::{Path, PathBuf}};
use view::*;
use utils::*;

fn validate(name: &str) -> io::Result<()> {
     let trimmed = name.trim();
     if trimmed.is_empty() || trimmed != name {
          return Err(io::Error::new(io::ErrorKind::InvalidInput, "instance name cannot be empty or have trailing whitepace"));
     }
     if name == "." || name == ".." || name.chars().count() > 64 {
          return Err(io::Error::new(io::ErrorKind::InvalidInput, "instance name is invalid or longer than 64 characters"));
     }
     if name.chars().any(|ch| ch.is_control() || matches!(ch, '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|')) {
          return Err(io::Error::new(io::ErrorKind::InvalidInput, "instance name contains unsupported characters"));
     }
     if name.ends_with('.') || name.ends_with(' ') {
          return Err(io::Error::new(io::ErrorKind::InvalidInput, "instance name cannot end with a dot or a space"));
     }
     let stem = name.split('.').next().unwrap_or(name).to_ascii_uppercase();
     let reserved = ["CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9"];
     if reserved.contains(&stem.as_str()) {
          return Err(io::Error::new(io::ErrorKind::InvalidInput, "instance name is reserved by the operating system"));
     }
     Ok(())
}

fn validate_version_id(id: &str) -> io::Result<()> {
     if id.is_empty() || id.len() > 128 || !id.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-')) {
          return Err(io::Error::new(io::ErrorKind::InvalidInput, "Minecraft version id is invalid"));
     }
     Ok(())
}

fn unique_name(i: &Path, d: &str) -> io::Result<(String, PathBuf)> {
     fs::create_dir_all(i)?;
     for counter in 0_u32.. {
          let can = if counter == 0 {
               d.to_owned()
          } else {
               format!("{} ({})", d, counter)
          };
          let path = i.join(&can);
          match fs::create_dir(&path) {
               Ok(()) => return Ok((can, path)),
               Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
               Err(error) => return Err(error),
          }
     }
     unreachable!("u32 instance-name counter exhaused")
}

pub async fn setup_instance(name: String, id: String, loader: String, loader_version: String) -> Result<(), Box<dyn Error>> {
     validate(&name)?;
     validate_version_id(&id)?;
     if !matches!(loader.as_str(), "vanilla" | "fabric" | "quilt" | "neoforge") {
          return Err(io::Error::new(io::ErrorKind::InvalidInput, "unsupported Minecraft loader").into())
     }
     if loader != "vanilla" && (loader_version.is_empty() || loader_version.len() > 128 || !loader_version.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-' | b'+'))) {
          return Err(io::Error::new(io::ErrorKind::InvalidInput, "unsupported Minecraft loader").into())
     }
     let home = check()?;
     let directory = home.join("instances");
     let (name_i, path) = unique_name(&directory, name.as_str())?;
     fs::create_dir_all(path.join("minecraft"))?;
     if let Err(err) = fetch_all_minecraft(&name_i, &loader, &loader_version, &id).await {
          let _ = fs::remove_dir_all(&path);
          let vers_path = home.join("versions").join(&id).join(name_i);
          let _ = fs::remove_dir_all(vers_path);
          return Err(err);
     }
     Ok(())
}