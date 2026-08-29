use serde::{Deserialize};
use reqwest::Result;

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

pub async fn fetch() -> Result<VersionManifest> {
     let resp = reqwest::get("https://launchermeta.mojang.com/mc/game/version_manifest_v2.json")
          .await?
          .json::<VersionManifest>()
          .await?;
     Ok(resp)
}

pub fn find_url<'a>(mani: &'a VersionManifest, id: &str) -> Option<&'a str> {
     mani.versions.iter().find(|v| v.id == id).map(|v| v.url.as_str())
}