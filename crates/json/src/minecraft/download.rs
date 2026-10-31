use std::{fs, io::{Read, Write}, path::{Path, PathBuf}, sync::atomic::{AtomicU64, Ordering}, time::Duration};
use anyhow::{Result, Context, bail};
use reqwest::Client;
use sha1::{Digest, Sha1};

static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

pub async fn download_to(client: &Client, url: &str, destination: &Path, expected: Option<u64>) -> Result<()> {
     download_to_verified(client, url, destination, expected, None).await
}

pub async fn download_to_verified(client: &Client, url: &str, dest: &Path, size: Option<u64>, sha1: Option<&str>) -> Result<()> {
     if file_match(dest, size, sha1) {
          return Ok(());
     }

     let parent = dest.parent().context("download destination has no parent")?;
     fs::create_dir_all(parent)?;
     let mut part = dest.as_os_str().to_owned();
     let seq = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
     part.push(format!(".{}.{}.part", std::process::id(), seq));
     let part_name = PathBuf::from(part);
     let mut attempt = 0;
     loop {
          let result: Result<()> = async {
               let mut resp = client.get(url).send().await?.error_for_status()?;
               let mut file = fs::File::create(&part_name)?;
               let mut hasher = Sha1::new();
               let mut downloaded = 0_u64;
               while let Some(chunk) = resp.chunk().await? {
                    downloaded += chunk.len() as u64;
                    if size.is_some_and(|s| downloaded > s) {
                         bail!("download exceeded expected size of {}", size.unwrap());
                    }
                    hasher.update(&chunk);
                    file.write_all(&chunk)?;
               }
               if let Some(size) = size && downloaded != size {
                    bail!("downloaded {}, expected {}", downloaded, size);
               }
               if let Some(expected) = sha1 {
                    let a = format!("{:x}", hasher.finalize());
                    if a != expected.to_ascii_lowercase() {
                         bail!("SHA-1 mismatch: expected {}, got {}", expected, a);
                    }
               }

               file.sync_all()?;
               Ok(())
          }.await;

          match result {
               Ok(()) => break,
               Err(error) if attempt >= 2 => {
                    let _ = fs::remove_file(&part_name);
                    return Err(error).with_context(|| format!("downloading {url}"));
               }
               Err(_) => {
                    let _ = fs::remove_file(&part_name);
                    attempt += 1;
                    tokio::time::sleep(Duration::from_millis(300 * attempt)).await;
               }
          }
     }
     #[cfg(windows)]
     if dest.exists() {
          fs::remove_file(dest).with_context(|| format!("removing old file at {}", dest.display()))?;
     }
     fs::rename(&part_name, dest).with_context(|| {
          format!("replacing downloaded file at {}", dest.display())
     })?;
     Ok(())
}

pub fn file_match(path: &Path, size: Option<u64>, sha1: Option<&str>) -> bool {
     let Ok(meta) = fs::metadata(path) else { return false; };
     if !meta.is_file() || size.is_some_and(|s| meta.len() != s) {
          return false;
     }
     match sha1 {
          Some(expected) => hash_file(path).is_ok_and(|a| a == expected.to_ascii_lowercase()),
          None => true,
     }
}

fn hash_file(path: &Path) -> Result<String> {
     let mut file = fs::File::open(path)?;
     let mut hasher = Sha1::new();
     let mut buf = [0_u8; 64 * 1024];
     loop {
          let count = file.read(&mut buf)?;
          if count == 0 { break; }
          hasher.update(&buf[..count]);
     }
     Ok(format!("{:x}", hasher.finalize()))
}