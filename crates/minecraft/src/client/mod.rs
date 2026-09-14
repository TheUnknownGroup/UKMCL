pub mod client_info;
pub mod assets;
pub mod jars;
pub mod window;
pub mod library;
pub mod offline;

use std::{error::Error, path::{PathBuf}, fs};
use serde::{Deserialize};

use crate::client::{assets::fetch_asset_index, jars::download_client_jar, library::fetch_libraries};
use folders::hub_fold::make_hub;
use config::Main;

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

pub async fn fetchs(app_handle: tauri::AppHandle, url: &str, dest_dir: PathBuf, inst_dir: &PathBuf, inst_name: &str) -> Result<ClientInfo, Box<dyn Error>> {
     let resp = reqwest::get(url)
          .await?
          .json::<ClientInfo>()
          .await?;

     let main = make_hub()?;
     let lib_dir = main.join("libraries");
     
     fetch_asset_index(app_handle, &resp.asset_index.url).await?;
     download_client_jar(&resp.downloads.client.url, dest_dir).await?;
     let paths =  fetch_libraries(&resp.libraries, &lib_dir).await?;

     let asset_dir = main.join("assets");
     let home = asset_dir.join("indexes");
     fs::create_dir_all(&home)?;

     let asset_json = reqwest::get(&resp.asset_index.url).await?;
     let bytes = asset_json.bytes().await?;
     let dest = home.join(format!("{}.json", resp.asset_index.id));
     fs::write(&dest, &bytes)?;

     let insts_dir = inst_dir.join("minecraft").to_string_lossy().to_string();
     let libs_dir = lib_dir.to_string_lossy().to_string();
     let assets_dir = asset_dir.to_string_lossy().to_string();
     let versions_dir = main.join("versions").join(&resp.id).join(inst_name).join("client.jar").to_string_lossy().to_string();

     let saves = Main::new(&inst_name, &resp.id, &resp.asset_index.id, &resp.main_class, &insts_dir, &libs_dir, &assets_dir, &paths, &versions_dir)?;
     saves.save(&inst_dir.join("config.toml").to_string_lossy().to_string())?;
     
     Ok(resp)
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