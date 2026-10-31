use std::{error::Error, fs, path::{Path, PathBuf}};
use json_minecraft::fetch_all_minecraft;
use tauri::AppHandle;
use utils::folders::make_hub;

fn unique_name(directory: &Path, desired: String) -> String {
     let current = directory.join(&desired);
     if !current.exists() {
          return desired;
     }

     let mut counter = 1;
     loop {
          let attempt = format!("{} ({})", desired, counter);
          if !directory.join(&attempt).exists() {
               return attempt
          }
          counter += 1
     }
}

pub async fn setup_instance(app_handle: AppHandle, name: String, id: String, loader: String, loader_version: String) -> Result<(), Box<dyn Error>> {
     let home = make_hub()?;
     let directory = home.join("instances");
     let new_name = unique_name(&directory, name);
     new_directories(&directory, &new_name).await?;
     fetch_all_minecraft(app_handle, &new_name, &loader, &loader_version, &id).await?;
     Ok(())
}

async fn new_directories(home: &PathBuf, name: &str) -> Result<(), Box<dyn Error>> {
     fs::create_dir_all(home.join(&name))?;
     if !home.join(&name).join("minecraft").exists() {
          fs::create_dir_all(home.join(&name).join("minecraft"))?;
     }
     Ok(())
}