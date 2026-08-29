use std::path::PathBuf;
use std::io;

pub fn make_main() -> io::Result<PathBuf> {
    let base = dirs::home_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "could not resolve data dir"))?;
    let root = base.join(".ukmcl");

    if !root.exists() {
        std::fs::create_dir_all(&root)?;
    }

    Ok(root)
}