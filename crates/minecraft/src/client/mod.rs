pub mod client_info;
pub mod assets;
pub mod jars;
pub mod window;

use std::{error::Error, path::{PathBuf}};

use serde::{Deserialize};

use crate::client::{assets::fetch_asset_index, jars::download_client_jar};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientInfo {
     pub main_class: String,
     pub asset_index: AssetIn,
     pub downloads: Downloads,
     pub libraries: Vec<Libraries>,
     pub java_version: JavaVersion,
}

#[derive(Deserialize, Debug)]
pub struct AssetIn {
     pub id: String,
     pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Downloads {
     pub client: ClientJar,
}

#[derive(Deserialize, Debug)]
pub struct ClientJar {
     pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Libraries {
     pub downloads: Artifact,
}

#[derive(Deserialize, Debug)]
pub struct Artifact {
     pub artifact: JarDownload,
}

#[derive(Deserialize, Debug)]
pub struct JarDownload {
     pub path: String,
     pub url: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JavaVersion {
     pub component: String,
     pub major_version: u32,
}

pub async fn fetchs(app_handle: tauri::AppHandle, url: &str, inst_dir: PathBuf) -> Result<ClientInfo, Box<dyn Error>> {
     let resp = reqwest::get(url)
          .await?
          .json::<ClientInfo>()
          .await?;

     fetch_asset_index(app_handle, &resp.asset_index.url).await?;
     download_client_jar(&resp.downloads.client.url, inst_dir).await?;
     
     Ok(resp)
}