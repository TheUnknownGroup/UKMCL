use std::{path::{Path}, collections::HashMap, error::Error, sync::{Arc, atomic::{AtomicU64, Ordering}}};
use tokio::{io::AsyncWriteExt, fs};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use reqwest::Client;
use futures_util::stream::{self, StreamExt};

use windows::download_bar::spawn;
use folders::hub_fold::make_hub;

#[derive(Deserialize, Debug)]
pub struct AssetIndex {
     pub objects: HashMap<String, AsObjects>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AsObjects {
     pub hash: String,
     pub size: u64,
}

#[derive(Clone, Serialize)]
struct DownloadProgress {
     downloaded: u64,
     total: u64,
     current: String,
     files: u32,
     file_total: u32,
}

pub async fn fetch_asset_index(app_handle: AppHandle, url: &str) -> Result<AssetIndex, Box<dyn Error>> {   
     let resp = reqwest::get(url)
          .await?
          .json::<AssetIndex>()
          .await?;

     let home = make_hub()?.join("assets");

     downloads(app_handle, &resp.objects, &home).await?;
     
     Ok(resp)
}

pub async fn downloads(app_handle: AppHandle, ind: &HashMap<String, AsObjects>, asset_path: &Path) -> Result<(), Box<dyn Error>> {
     let obj_dir = asset_path.join("objects");
     let pending: Vec<(String, AsObjects)> = ind
          .iter()
          .filter(|(_, obj)| {
               let pref = &obj.hash[0..2];
               let dest = obj_dir.join(pref).join(&obj.hash);
               !dest.exists()
          })
          .map(|(name, obj)| (name.clone(), obj.clone()))
          .collect();

     let totals: u64 = pending.iter().map(|(_, obj)| obj.size).sum();
     let totalf = pending.len() as u32;
     
     if totalf == 0 {
          return Ok(());
     }
     spawn(&app_handle).await?;
     
     let client = Client::new();
     let downloaded = Arc::new(AtomicU64::new(0));
     let files = Arc::new(AtomicU64::new(0));

     let handle = app_handle.clone();
     let progress = {
          move |
          app_handle: &AppHandle,
          download: u64,
          file: u32,
          name: String| -> Result<(), Box<dyn Error + Send + Sync >> {
               app_handle.emit("progress", DownloadProgress {
                    downloaded: download,
                    total: totals,
                    current: name,
                    files: file,
                    file_total: totalf,
               })?;
               Ok(())
          }
     };

     let progress = Arc::new(progress);
     
     stream::iter(pending)
        .map(move |(name, obj)| {
             let client = client.clone();
             let app_handle = handle.clone();
             let downloaded = downloaded.clone();
             let files = files.clone();
             let obj_dir = obj_dir.clone();
             let progress = progress.clone();

             async move {
                  let pref = &obj.hash[0..2];
                  let dest = obj_dir.join(pref).join(&obj.hash);
                  std::fs::create_dir_all(dest.parent().unwrap())?;
                  let url = format!(
                       "https://resources.download.minecraft.net/{}/{}",
                       pref, obj.hash
                  );
                  let bytes = client.get(&url).send().await?;
                  let mut stream = bytes.bytes_stream();
                  let mut file = fs::File::create(&dest).await?;

                  let thres: u64 = 65536;
                  let mut since: u64 = 0;

                  while let Some(chunk) = stream.next().await {
                       let chunk = chunk?;
                       file.write_all(&chunk).await?;
                       let chunk_len = chunk.len() as u64;
                       since += chunk_len;
                       let downloadeds = downloaded.fetch_add(chunk_len, Ordering::Relaxed) + chunk_len;

                       if since >= thres {
                            progress(
                                 &app_handle,
                                 downloadeds,
                                 files.load(Ordering::Relaxed) as u32,
                                 name.clone(),
                            )?;
                            since = 0;
                       }
                  }
        
                  let filess = files.fetch_add(1, Ordering::Relaxed) as u32 + 1;

                  progress(
                       &app_handle,
                       downloaded.load(Ordering::Relaxed),
                       filess,
                       name.clone(),
                  )?;

                  Ok::<(), Box<dyn Error + Send + Sync>>(())
             }
        })
        .buffer_unordered(6)
        .for_each(|result| async {
             if let Err(e) = result {
                  eprintln!("asset download failed for {e}");
             }
        })
        .await;
     
     app_handle.emit("complete", ())?;
     
     Ok(())     
}