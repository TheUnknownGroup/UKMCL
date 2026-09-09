use crate::req::{VersionEntry, fetch};
use std::error::Error;

pub async fn fetches(ver: &str) -> Result<VersionEntry, Box<dyn Error>> {
     let mani = fetch().await?;
     mani.versions.iter().find(|v| v.id == ver).cloned().ok_or_else(|| format!("version not found: {}", ver).into())
}