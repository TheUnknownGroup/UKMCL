use clients::base_client;
use serde::{Deserialize};
use reqwest::Result;

const VERSION_MANIFEST_URL: &str = "https://launchermeta.mojang.com/mc/game/version_manifest_v2.json";
const FABRIC_URL_GAME: &str = "https://meta.fabricmc.net/v2/versions/game";
const FABRIC_URL_LOADER: &str = "https://meta.fabricmc.net/v2/versions/loader";
const QUILT_URL_GAME: &str = "https://meta.quiltmc.org/v3/versions/game";
const QUILT_URL_LOADER: &str = "https://meta.quiltmc.org/v3/versions/loader";

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

pub async fn fetch() -> Result<VersionMani> {
     let client = base_client();
     let resp = client.get(VERSION_MANIFEST_URL).send()
          .await?
          .json::<VersionMani>()
          .await?;

     Ok(resp)
}

#[derive(Deserialize, Clone, Debug)]
pub struct FabricGameVersions {
     pub version: String,
     pub stable: bool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct FabLoader {
     pub version: String,
     pub stable: bool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct QuiltLoader {
     pub version: String,
}

pub async fn fabric() -> Result<Vec<FabricGameVersions>> {
     let client = base_client();
     let resp = client.get(FABRIC_URL_GAME).send()
          .await?
          .json::<Vec<FabricGameVersions>>()
          .await?;
     Ok(resp)
}

pub async fn fabric_load() -> Result<Vec<FabLoader>> {
     let client = base_client();
     let resp = client.get(FABRIC_URL_LOADER).send()
          .await?
          .json::<Vec<FabLoader>>()
          .await?;
     Ok(resp)
}

pub async fn quilt() -> Result<Vec<FabricGameVersions>> {
     let client = base_client();
     let resp = client.get(QUILT_URL_GAME).send()
          .await?
          .json::<Vec<FabricGameVersions>>()
          .await?;
     Ok(resp)
}

pub async fn quilt_load() -> Result<Vec<QuiltLoader>> {
     let client = base_client();
     let resp = client.get(QUILT_URL_LOADER).send()
          .await?
          .json::<Vec<QuiltLoader>>()
          .await?;
     Ok(resp)
}