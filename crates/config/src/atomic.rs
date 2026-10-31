use std::{
     fs::{self, OpenOptions},
     io::{self, Write},
     path::{Path, PathBuf},
     time::{SystemTime, UNIX_EPOCH},
};
pub(crate) fn atomic_write(path: &Path, contents: &[u8]) -> io::Result<()> {
     let parent = path.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or(Path::new("."));
     fs::create_dir_all(parent)?;
     let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis();
     let mut temp_name = path.as_os_str().to_owned();
     temp_name.push(format!(".{}.{}.tmp", std::process::id(), stamp));
     let temp = PathBuf::from(temp_name);
     let result = (|| {
          let mut opts = OpenOptions::new();
          opts.write(true).create_new(true);
          #[cfg(unix)]
          {
               use std::os::unix::fs::OpenOptionsExt;
               opts.mode(0o600);
          }
          let mut file = opts.open(&temp)?;
          file.write_all(contents)?;
          file.sync_all()?;

          #[cfg(windows)]
          if path.exists() {
               fs::remove_file(path)?;
          }
          fs::rename(&temp, path)
     })();

     if result.is_err() {
          let _ =fs::remove_dir(&temp);
     }
     result
}