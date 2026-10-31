use std::{collections::HashMap, path::Path, sync::{Arc, atomic::{AtomicU64, Ordering::Relaxed}}, time::Instant};
use futures_util::stream::{self, StreamExt};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use crate::minecraft::download::download_to;

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

pub async fn asset_index(url: &str, index_path: &Path, client: &reqwest::Client) -> anyhow::Result<AssetIndex> {
     let bytes = client.get(url).send().await?.error_for_status()?.bytes().await?;
     let index: AssetIndex = serde_json::from_slice(&bytes)?;
     if let Some(dir) = index_path.parent() {
          tokio::fs::create_dir_all(dir).await?;
     }
     tokio::fs::write(index_path, &bytes).await?;
     Ok(index)
}

pub async fn download_assets(app_handle: AppHandle, asset_path: &Path, index: &HashMap<String, AsObjects>, client: &reqwest::Client) -> anyhow::Result<()> {
     let obj_dir = asset_path.join("objects");
     let pending: Vec<(String, AsObjects)> = index.iter()
          .filter(|(_, o)| {
               let dest = obj_dir.join(&o.hash[..2]).join(&o.hash);
               std::fs::metadata(&dest).map_or(true, |m| m.len() != o.size)
          })
          .map(|(m, o)| (m.clone(), o.clone()))
          .collect();

     let totals: u64 = pending.iter().map(|(_, obj)| obj.size).sum();
     let totalf = pending.len() as u32;
     
     let (downloaded, files, last_emit) = (Arc::new(AtomicU64::new(0)), Arc::new(AtomicU64::new(0)), Arc::new(AtomicU64::new(0)));
     let started = Instant::now();
     let result: Vec<Result<(), String>> = stream::iter(pending)
          .map(|(name, obj)| {
               let (client, app) = (client.clone(), app_handle.clone());
               let (downloaded, files, last_emit) = (downloaded.clone(), files.clone(), last_emit.clone());
               let obj_dir = &obj_dir;

               async move {
                    let pref = &obj.hash[..2];
                    let dest = obj_dir.join(pref).join(&obj.hash);
                    let url = format!("https://resources.download.minecraft.net/{}/{}", pref, obj.hash);
                    let res = download_to(&client, &url, &dest, Some(obj.size)).await;
                    if res.is_ok() {
                         let d = downloaded.fetch_add(obj.size, Relaxed) + obj.size;
                         let f = files.fetch_add(1, Relaxed) + 1;
                         let now = started.elapsed().as_millis() as u64;
                         let prev = last_emit.load(Relaxed);
                         if now.saturating_sub(prev) >= 100 && last_emit.compare_exchange(prev, now, Relaxed, Relaxed).is_ok() {
                              let _ = app.emit("progress", DownloadProgress {
                                   downloaded: d, total: totals, current: name.clone(), files: f as u32, file_total: totalf,
                              });
                         }
                    }

                    res.map_err(|e| format!("{}: {e:#}", name))
               }
          }).buffer_unordered(10).collect().await;

     let failed: Vec<_> = result.into_iter().filter_map(Result::err).collect();
     if !failed.is_empty() {
          anyhow::bail!("{} asset download failed, first: {}", failed.len(), failed[0])
     }
     app_handle.emit("progress", DownloadProgress {
          downloaded: totals, total: totals, current: String::new(), files: totalf, file_total: totalf
     })?;
     app_handle.emit("complete", ())?;
     
     Ok(())
}