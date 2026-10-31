use utils::client::base;
use serde::Deserialize;
use reqwest::Result;

const VERSION_MANIFEST_URL: &str = "https://launchermeta.mojang.com/mc/game/version_manifest_v2.json";

#[derive(Deserialize)]
pub struct VersionManifest {
     pub versions: Vec<VersionEntry>,
}

#[derive(Deserialize, Clone)]
pub struct VersionEntry {
     pub id: String,
     #[serde(rename = "type")]
     pub ver_type: String,
     pub url: String,
}

pub async fn fetch_vanilla_list() -> Result<VersionManifest> {
     let client = base();
     let resp = client.get(VERSION_MANIFEST_URL).send().await?.json::<VersionManifest>().await?;
     Ok(resp)
}