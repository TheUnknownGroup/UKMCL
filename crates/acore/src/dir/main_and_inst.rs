use std::path::PathBuf;
use std::path::Path;
use tokio::*;

pub fn make_main() -> io::Result<PathBuf> {
    let base = dirs::home_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "could not resolve data dir"))?;
    let root = base.join(".ukmcl");

    if !root.exists() {
        std::fs::create_dir_all(&root)?;
    }

    Ok(root)
}

pub fn check_dir() -> io::Result<PathBuf> {
    let root = make_main()?;
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

    // for sub in ["minecraft"] {
    //     std::fs::create_dir_all(&instance_dir.join(sub))?;
    // }

    Ok(instance_dir)
}