use std::io;

use crate::dir::main_and_inst::check_dir;

pub fn list() -> io::Result<Vec<String>> {
    let instances_dir = check_dir()?;

    if !instances_dir.is_dir() {
        return Ok(Vec::new());
    }

    let mut names = Vec::new();
    for entry in std::fs::read_dir(&instances_dir)? {
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