use serde::{Deserialize};

const VERSION_MANIFEST_URL: &str = "https://launchermeta.mojang.com/mc/game/version_manifest_v2.json";

#[derive(Deserialize)]
pub struct VersionMani {
     pub versions: Vec<VersionEntry>,
}

#[derive(Deserialize, Clone)]
pub struct VersionEntry {
     pub id: String,
     #[serde(rename = "type")]
     pub ver_type: String,
     pub url: String,
}

pub async fn fetch() -> reqwest::Result<VersionMani> {
     let resp = reqwest::get(VERSION_MANIFEST_URL)
          .await?
          .json::<VersionMani>()
          .await?;

     Ok(resp)
}