use std::{fs, path::{Path, PathBuf}, time::Duration};
use anyhow::Result;
use reqwest::Client;

pub async fn download_to(client: &Client, url: &str, destination: &Path, expected: Option<u64>) -> Result<()> {
     if let Ok(meta) = fs::metadata(destination) && expected.map_or(true, |s| s == meta.len()) {
          return Ok(());
     }

     if let Some(dir) = destination.parent() {
          fs::create_dir_all(dir)?;
     }
     let mut part = destination.as_os_str().to_owned();
     part.push(".part");
     let part = PathBuf::from(part);
     let mut attempt = 0;
     loop {
          let res: Result<()> = async {
               let bytes = client.get(url).send().await?.error_for_status()?.bytes().await?;
               fs::write(&part, &bytes)?;
               Ok(())
          }.await;

          match res {
               Ok(()) => break,
               Err(e) if attempt >= 2 => return Err(e.context(format!("downloading {url}"))),
               Err(_) => {
                    attempt += 1;
                    tokio::time::sleep(Duration::from_millis(300 * attempt)).await;
               }
          }
     }
     fs::rename(&part, destination)?;
     Ok(())
}