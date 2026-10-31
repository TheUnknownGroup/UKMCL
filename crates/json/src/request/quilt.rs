use crate::request::fabric::FabricGameVersions;
use serde::Deserialize;
use reqwest::Result;
use utils::client::base;

const QUILT_URL_GAME: &str = "https://meta.quiltmc.org/v3/versions/game";
const QUILT_URL_LOADER: &str = "https://meta.quiltmc.org/v3/versions/loader";

#[derive(Deserialize, Clone, Debug)]
pub struct QuiltLoader {
     pub version: String,
}

pub async fn quilt_version() -> Result<Vec<FabricGameVersions>> {
     let client = base();
     let resp = client.get(QUILT_URL_GAME).send()
          .await?
          .json::<Vec<FabricGameVersions>>()
          .await?;
     Ok(resp)
}

pub async fn quilt_loader() -> Result<Vec<QuiltLoader>> {
     let client = base();
     let resp = client.get(QUILT_URL_LOADER).send()
          .await?
          .json::<Vec<QuiltLoader>>()
          .await?;
     Ok(resp)
}