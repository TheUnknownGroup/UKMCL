use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct ClientManifest {
    pub libraries: Vec<Library>,
    #[serde(rename = "assetIndex")]
    pub asset_index: AssetIndex,
}

#[derive(Deserialize)]
pub struct Library {
    pub downloads: DownloadLib,
}

#[derive(Deserialize)]
pub struct AssetIndex {
    pub url: String,
}

#[derive(Deserialize)]
pub struct DownloadLib {
    pub artifact: Vec<Artifact>,
}

#[derive(Deserialize)]
pub struct Artifact {
    pub path: PathBuf,
    pub url: String,
}

// pub async fn fetch_client_info() -> 