use reqwest::Result;
use serde::Deserialize;
use utils::client::base;

const FABRIC_URL_GAME: &str = "https://meta.fabricmc.net/v2/versions/game";
const FABRIC_URL_LOADER: &str = "https://meta.fabricmc.net/v2/versions/loader";

#[derive(Deserialize, Clone, Debug)]
pub struct FabricGameVersions {
     pub version: String,
     pub stable: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FabricLoader {
     pub version: String,
     pub stable: bool,
}

pub async fn fabric_version() -> Result<Vec<FabricGameVersions>> {
     let client = base();
     let resp = client.get(FABRIC_URL_GAME).send().await?.json::<Vec<FabricGameVersions>>().await?;
     Ok(resp)
}

pub async fn fabric_loader() -> Result<Vec<FabricLoader>> {
     let client = base();
     let resp = client.get(FABRIC_URL_LOADER).send().await?.json::<Vec<FabricLoader>>().await?;
     Ok(resp)
}