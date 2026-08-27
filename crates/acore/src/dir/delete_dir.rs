use crate::dir::main_and_inst::check_dir;
use std::io;

pub fn delete_inst(instance_name: &str) -> io::Result<()> {
    let instances_dir = check_dir()?;
    let instance_dir = instances_dir.join(instance_name);
    if !instance_dir.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("instance not found: {}", instance_name),
        ));
    }
    std::fs::remove_dir_all(&instance_dir)?;
    Ok(())
}