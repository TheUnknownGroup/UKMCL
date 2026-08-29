use std::path::PathBuf;
use std::path::Path;
use tokio::*;

use crate::dir::main_sub::add_sub_dirs;

use download::client::fetch_client;

use types::main_dir::make_main;

pub fn check_dir() -> io::Result<PathBuf> {
    let root = make_main()?;
    add_sub_dirs()?;
    let instances = root.join("instances");

    if !root.exists() {
        std::fs::create_dir_all(&instances)?;
    } else if !instances.exists() {
        std::fs::create_dir_all(&instances)?;
    } else if !root.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
                format!("expected directory not found: {}", root.display()),
        ))
    }

    Ok(instances)
}

fn unique_name(instances_dir: &Path, desired_name: &str) -> String {
    let can = instances_dir.join(desired_name);
    if !can.exists(){
        return desired_name.to_string();
    }

    let mut counter = 1;
    loop {
        let attempt = format!("{} ({})", desired_name, counter);
        if !instances_dir.join(&attempt).exists() {
            return attempt
        }
        counter += 1;
    }
}

fn get_inst_dir(instance_name: &str) -> io::Result<PathBuf> {
    let instances_dir = check_dir()?;
    let unique_name = unique_name(&instances_dir, instance_name);
    let instance_dir = instances_dir.join(&unique_name);

    std::fs::create_dir_all(&instance_dir)?;
    
    Ok(instance_dir)
}

pub fn setup_instance(instance_name: &str) -> io::Result<PathBuf> {
    let instance_dir = get_inst_dir(instance_name)?;

    let sub_dir = instance_dir.join("version");
    std::fs::create_dir_all(&sub_dir)?;
    
    Ok(instance_dir)
}

pub async fn setup_mani(inst_name: &str, id: &str) -> std::result::Result<PathBuf, Box<dyn std::error::Error>> {
    let inst_dir = setup_instance(inst_name)?;
    let sub_dir = inst_dir.join("version");

    fetch_client(&id.to_string(), &sub_dir).await.map_err(|e| e.to_string())?;

    Ok(inst_dir)
}