pub mod request;
pub mod minecraft;

use std::{error::Error, fs, path::Path};

use config::write::{InstanceCfg, instance_config};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use tauri::AppHandle;
use utils::{client::base, folders::make_hub};

use crate::{minecraft::{ClientInfo, Fabric, Quilt, assets::{asset_index, download_assets}, download::download_to, library::{fabric_libraries, fetch_libraries}, version::get_version_info}, request::{fabric::{fabric_loader, fabric_version}, quilt::{quilt_loader, quilt_version}, vanilla::fetch_vanilla_list}};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ListEntry {
     pub id: String,
     pub ver_type: Option<String>,
}

pub async fn make_version_json(loader: &str) -> Result<(), Box<dyn Error>> {
     let home = make_hub()?.join("jsons");
     let all: Vec<ListEntry> = match loader {
          "vanilla" => {
               let mani = fetch_vanilla_list().await?;
               mani.versions.iter().map(|v| {
                    ListEntry { id: v.id.clone(), ver_type: Some(v.ver_type.clone()) }
               }).collect()
          },
          "fabric" => {
               let mani = fabric_version().await?;
               mani.into_iter().map(|g| {
                    ListEntry { id: g.version, ver_type: Some("".into()) }
               }).collect()
          },
          "quilt" => {
               let mani = quilt_version().await?;
               mani.into_iter().map(|g| {
                    ListEntry { id: g.version, ver_type: Some("".into()) }
               }).collect()
          }
          "neoforge" => {
               let mani = fetch_vanilla_list().await?;
               mani.versions.iter().map(|v| {
                    ListEntry { id: v.id.clone(), ver_type: Some(v.ver_type.clone()) }
               }).collect()
          },
          "forge" => {
               let mani = fetch_vanilla_list().await?;
               mani.versions.iter().map(|v| {
                    ListEntry { id: v.id.clone(), ver_type: Some(v.ver_type.clone()) }
               }).collect()
          },
          other => return Err(format!("unknown loader: {}", other).into()),
     };
     let loaders: Option<Vec<String>> = match loader {
          "vanilla" => None,
          "fabric" => Some(fabric_loader().await?.into_iter().map(|g| g.version).collect()),
          "quilt" => Some(quilt_loader().await?.into_iter().map(|g| g.version).collect()),
          "neoforge" => None,
          "forge" => None,
          other => return Err(format!("Unknown loader found: {}", other).into())
     };

     let file = if loader == "vanilla" || loader == "neoforge" || loader == "forge" {
          "versions.json".to_string()
     } else {
          format!("{}.json", loader)
     };
     write(&home, &file, &all)?;
     if let Some(list) = loaders {
          write(&home, file_find(loader)?, &list)?;
     }

     Ok(())
}

pub fn file_find(loader: &str) -> Result<&'static str, String> {
     match loader {
          "vanilla" => Err("Vanilla is it's own loader".to_string()),
          "fabric" => Ok("fabric_loader.json"),
          "quilt" => Ok("quilt_loader.json"),
          "neoforge" => Err("Neoforge currently unavailable".to_string()),
          "forge" => Err("Forge currently unavailable".to_string()),
          other => Err(format!("unknown loader: {}", other)),
     }
}

pub fn write<T>(home: &Path, file: &str, data: &[T]) -> Result<(), Box<dyn Error>> where T: Serialize + DeserializeOwned + PartialEq {
     let path = home.join(file);
     let unchanged = std::fs::read_to_string(&path)
          .ok()
          .and_then(|raw| serde_json::from_str::<Vec<T>>(&raw).ok())
          .is_some_and(|x| x.as_slice() == data);
     if !unchanged {
          std::fs::write(&path, serde_json::to_string_pretty(data)?)?;
     }
     Ok(())
}

pub struct Resolved {
     main_class: String,
     classpath: Vec<String>,
     jvm_args: Vec<String>,
     game_args: Vec<String>,
}

pub async fn fetch_all_minecraft(app: AppHandle, name: &str, loader: &str, loader_version: &str, id: &str) -> Result<ClientInfo, Box<dyn Error>> {
     let client = base();
     let home = make_hub()?;
     let url = get_version_info(id).await?.url;
     let raw = client.get(url).send().await?.error_for_status()?.bytes().await?;
     let resp: ClientInfo = serde_json::from_slice(&raw)?;
     let assets_directory = home.join("assets");
     let index_path = assets_directory.join("indexes").join(format!("{}.json", resp.asset_index.id));
     let index = asset_index(&resp.asset_index.url, &index_path, &client).await?;
     let version_directory = home.join("versions").join(&id).join(name);
     fs::create_dir_all(&version_directory)?;
     let client_jar = version_directory.join("client.jar");
     download_assets(app, &assets_directory, &index.objects, &client).await?;
     download_to(&client, &resp.downloads.client.url, &client_jar, resp.downloads.client.size).await?;

     let resolved = match loader {
          "vanilla" => Resolved {
               main_class: resp.main_class.clone(),
               classpath: fetch_libraries(&resp.libraries, &home).await?,
               jvm_args: Vec::new(),
               game_args: Vec::new(),
          },
          "fabric" => fab(&resp, loader_version, &home).await?,
          "quilt" => qui(&resp, loader_version, &home).await?,
          other => return Err(format!("couldn't find launcher: {}", other).into())
     };

     instance_config(&InstanceCfg {
          name,
          mc_id: &resp.id,
          asset_index: &resp.asset_index.id,
          main_class: &resolved.main_class,
          hub: &home,
          classpath: &resolved.classpath,
          loader,
          loader_ver: loader_version,
          component: &resp.java_version.component,
          supports_quick_play_single: resp.supports("is_quick_play_singleplayer"),
          supports_quick_play_multi: resp.supports("is_quick_play_multiplayer"),
          jvm_args: &resolved.jvm_args,
          game_args: &resolved.game_args,
     })?;
     
     Ok(resp)
}

pub async fn fab(resp: &ClientInfo, loader_version: &str, home: &Path) -> Result<Resolved, Box<dyn Error>> {
     let url = format!("https://meta.fabricmc.net/v2/versions/loader/{}/{}", resp.id, loader_version);
     let body = base().get(url).send().await?.error_for_status()?.text().await?;
     let meta: Fabric = serde_json::from_str(&body).map_err(|e| format!("decode failed: {}", e))?;

     let classpath = fabric_libraries(&meta.launcher_meta.libraries.common, &meta.loader.maven, home, &meta.intermediary.maven, &resp.libraries, "fabric").await?;
     Ok(Resolved {
          main_class: meta.launcher_meta.main_class.client,
          classpath,
          jvm_args: Vec::new(),
          game_args: Vec::new(),
     })
}

pub async fn qui(resp: &ClientInfo, loader_version: &str, home: &Path) -> Result<Resolved, Box<dyn Error>> {
     let url = format!("https://meta.quiltmc.org/v3/versions/loader/{}/{}", resp.id, loader_version);
     let body = base().get(url).send().await?.error_for_status()?.text().await?;
     let meta: Quilt = serde_json::from_str(&body).map_err(|e| format!("decode failed: {}", e))?;

     let classpath = fabric_libraries(&meta.launcher_meta.libraries.common, &meta.loader.maven, home, "", &resp.libraries, "quilt").await?;
     Ok(Resolved {
          main_class: meta.launcher_meta.main_class.client,
          classpath,
          jvm_args: Vec::new(),
          game_args: Vec::new(),
     })
}