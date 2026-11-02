use std::{collections::{HashMap, HashSet}, path::Path, sync::{Arc, atomic::{AtomicU64, Ordering::Relaxed}}};
use futures_util::stream::{self, StreamExt};
use serde::{Deserialize};
use tauri::{AppHandle};
use utils::progress::emit_progress;
use crate::minecraft::download::{download_to_verified, file_match};

#[derive(Deserialize, Debug)]
pub struct AssetIndex {
     pub objects: HashMap<String, AsObjects>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AsObjects {
     pub hash: String,
     pub size: u64,
}

pub async fn asset_index(url: &str, index_path: &Path, client: &reqwest::Client, size: Option<u64>, sha1: Option<&str>) -> anyhow::Result<AssetIndex> {
     download_to_verified(client, url, index_path, size, sha1).await?;
     let bytes = tokio::fs::read(index_path).await?;
     Ok(serde_json::from_slice(&bytes)?)
}

pub async fn download_assets(app_handle: AppHandle, asset_path: &Path, index: &HashMap<String, AsObjects>, client: &reqwest::Client) -> anyhow::Result<()> {
     let obj_dir = asset_path.join("objects");
     let mut seen = HashSet::new();
     let pending: Vec<(String, AsObjects)> = index.iter()
          .filter(|(_, obj)| seen.insert(obj.hash.clone()))
          .filter(|(_, o)| {
               if o.hash.len() != 40 || !o.hash.bytes().all(|b| b.is_ascii_hexdigit()) {
                    return true;
               }
               let dest = obj_dir.join(&o.hash[..2]).join(&o.hash);
               !file_match(&dest, Some(o.size), Some(&o.hash))
          })
          .map(|(m, o)| (m.clone(), o.clone()))
          .collect();

     let totals: u64 = pending.iter().map(|(_, obj)| obj.size).sum();
     let totalf = pending.len() as u32;
     let downloaded = Arc::new(AtomicU64::new(0));
     let files = Arc::new(AtomicU64::new(0));
     emit_progress(&app_handle, "assets", 0, totals, "Preparing assets", 0, totalf)?;
     let result: Vec<Result<(), String>> = stream::iter(pending)
          .map(|(name, obj)| {
               let client = client.clone();
               let app = app_handle.clone();
               let downloaded = downloaded.clone();
               let files = files.clone();
               let obj_dir = obj_dir.clone();

               async move {
                    if obj.hash.len() != 40 || !obj.hash.bytes().all(|b| b.is_ascii_hexdigit()) {
                         return Err(format!("{}: invalid SHA-1 asset hash", name));
                    }
                    let pref = &obj.hash[..2];
                    let dest = obj_dir.join(pref).join(&obj.hash);
                    let url = format!("https://resources.download.minecraft.net/{}/{}", pref, obj.hash);
                    let res = download_to_verified(&client, &url, &dest, Some(obj.size), Some(&obj.hash)).await;
                    if res.is_ok() {
                         let d = downloaded.fetch_add(obj.size, Relaxed) + obj.size;
                         let f = files.fetch_add(1, Relaxed) + 1;
                         let _ = emit_progress(&app, "assets", d, totals, name.clone(), f as u32, totalf);
                    }

                    res.map_err(|e| format!("{}: {e:#}", name))
               }
          }).buffer_unordered(10).collect().await;

     let failed: Vec<_> = result.into_iter().filter_map(Result::err).collect();
     if !failed.is_empty() {
          anyhow::bail!("{} asset download failed, first: {}", failed.len(), failed[0]);
     }
     emit_progress(&app_handle, "assets", totals, totals, "Assets complete", totalf, totalf)?;
     
     Ok(())
}