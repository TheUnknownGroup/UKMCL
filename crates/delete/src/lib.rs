use create::create_hub::check_dir;
use std::io::Result;
use std::io::Error;
use std::io::ErrorKind;
use std::fs;
use config::Main;
use folders::hub_fold::make_hub;

pub fn inst_del(inst_name: &str) -> Result<()> {
     let insts_dir = check_dir()?;
     let inst_dir = insts_dir.join(inst_name);

     let config = inst_dir.join("config.toml");
     let info = Main::load(&config.to_string_lossy().to_string())?;
     let version = &info.main.minecraft_version;
     let version_dir = make_hub()?.join("versions").join(version).join(inst_name);

     fs::remove_dir_all(version_dir)?;
     
     if !inst_dir.is_dir() {
          return Err(Error::new(ErrorKind::NotFound,
               format!("inst not found: {}", inst_name),
          ));
     }
     fs::remove_dir_all(&inst_dir)?;
     
     Ok(())
}