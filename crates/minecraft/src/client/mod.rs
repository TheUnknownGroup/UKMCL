pub mod client_info;
pub mod assets;
pub mod jars;
pub mod library;

use std::{error::Error, path::{PathBuf}, fs};
use serde::{Deserialize};

use crate::client::{assets::fetch_asset_index, jars::download_client_jar, library::{fabric_libraries, fetch_libraries}};
use folders::hub_fold::make_hub;
use config::write::cfg_write;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientInfo {
     pub main_class: String,
     pub asset_index: AssetIn,
     pub downloads: Downloads,
     pub libraries: Vec<Libraries>,
     pub java_version: JavaVersion,
     pub id: String,
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
     pub name: String,
     pub downloads: Artifact,
     pub rules: Option<Vec<Rule>>,
}

#[derive(Deserialize, Debug)]
pub struct Rule {
     pub action: String,
     pub os: Option<Os>,
}

#[derive(Deserialize, Debug)]
pub struct Os {
     pub name: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct Artifact {
     pub artifact: Option<JarDownload>,
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Fabric {
     pub loader: LoadInfo,
     pub launcher_meta: LauncherMeta,
     pub intermediary: Intermediary,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Quilt {
     pub loader: LoadInfo,
     pub launcher_meta: LauncherMeta,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadInfo {
     pub maven: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Intermediary {
     pub maven: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LauncherMeta {
     pub libraries: Libs,
     pub main_class: MainClass,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Libs {
     pub common: Vec<CommonLibs>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommonLibs {
     pub name: String,
     pub url: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MainClass {
     pub client: String,
}

pub async fn fetchs(app_handle: tauri::AppHandle, url: &str, dest_dir: PathBuf, inst_name: &str, loader: &str, loader_ver: &str) -> Result<ClientInfo, Box<dyn Error>> {
     let resp = reqwest::get(url)
          .await?
          .json::<ClientInfo>()
          .await?;
     
     let main = make_hub()?;
  
     fetch_asset_index(app_handle, &resp.asset_index.url).await?;
     download_client_jar(&resp.downloads.client.url, dest_dir).await?;
     
     let home = &main.join("assets").join("indexes");

     let asset_json = reqwest::get(&resp.asset_index.url).await?;
     let bytes = asset_json.bytes().await?;
     let dest = home.join(format!("{}.json", resp.asset_index.id));
     fs::write(&dest, &bytes)?;

     match loader {
          "vanilla" => {
               let paths = fetch_libraries(&resp.libraries, &main).await?;
               cfg_write(&inst_name, &resp.id, &resp.asset_index.id, &resp.main_class, main.clone(), &paths, loader, loader_ver)?;
          }
          "fabric" => {
               fab(&resp.id, loader_ver, &resp.asset_index.id, main.clone(), inst_name, &resp.libraries, loader).await?;
          },
          "quilt" => {
               qui(&resp.id, loader_ver, &resp.asset_index.id, main.clone(), inst_name, &resp.libraries, loader).await?;
          }
          other => return Err(format!("couldn't find launcher: {}", other).into())
     }
     
     Ok(resp)
}

async fn fab(id: &str, loader_ver: &str, asset_id: &str, main: PathBuf, inst_name: &str, libs: &[Libraries], loader: &str) -> Result<(), Box<dyn Error>> {
     let resp2 = reqwest::get(format!("https://meta.fabricmc.net/v2/versions/loader/{}/{}", &id, loader_ver)).await?;
     let body = resp2.text().await?;
     let as_json: Fabric = serde_json::from_str(&body).map_err(|e| { format!("decode failed: {e}") })?;
     let paths = fabric_libraries(&as_json.launcher_meta.libraries.common, &as_json.loader.maven, &main, &as_json.intermediary.maven, libs, loader).await?;
     cfg_write(&inst_name, &id, &asset_id, &as_json.launcher_meta.main_class.client, main, &paths, loader, loader_ver)?;

     Ok(())
}

async fn qui(id: &str, loader_ver: &str, asset_id: &str, main: PathBuf, inst_name: &str, libs: &[Libraries], loader: &str) -> Result<(), Box<dyn Error>> {
     let resp2 = reqwest::get(format!("https://meta.quiltmc.org/v3/versions/loader/{}/{}", &id, loader_ver)).await?;
     let body = resp2.text().await?;
     let as_json: Quilt = serde_json::from_str(&body).map_err(|e| { format!("decode failed: {e}") })?;
     let paths = fabric_libraries(&as_json.launcher_meta.libraries.common, &as_json.loader.maven, &main, "", libs, loader).await?;
     cfg_write(&inst_name, &id, &asset_id, &as_json.launcher_meta.main_class.client, main, &paths, loader, loader_ver)?;

     Ok(())
}

pub fn current_os() -> &'static str {
     if cfg!(target_os = "windows") { "windows" }
     else if cfg!(target_os = "macos") { "osx" }
     else { "linux" }
}

pub fn applies(rules: &Option<Vec<Rule>>) -> bool {
     match rules {
          None => true,
          Some(rs) => rs.iter().all(|r| {
               let os_match = r.os.as_ref()
                    .and_then(|o| o.name.as_deref())
                    .map(|n| n == current_os())
                    .unwrap_or(true);
               match r.action.as_str() {
                    "allow" => os_match || r.os.is_none(),
                    "disallow" => !os_match,
                    _ => true,
               }
          })
     }
}