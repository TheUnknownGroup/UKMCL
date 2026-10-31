use std::error::Error;

use crate::request::vanilla::{VersionEntry, fetch_vanilla_list};

pub async fn get_version_info(version: &str) -> Result<VersionEntry, Box<dyn Error>> {
     let manifest = fetch_vanilla_list().await?;
     manifest.versions.iter().find(|v| v.id == version).cloned().ok_or_else(|| format!("version not found {}", version).into())
}