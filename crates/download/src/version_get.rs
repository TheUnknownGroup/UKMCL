use get::versions_get::fetch;
use get::versions_get::VersionEntry;


pub async fn fetch_version(version: &str) -> Result<VersionEntry, Box<dyn std::error::Error>> {
     let mani = fetch().await?;
     mani.versions.iter().find(|v| v.id == version).cloned().ok_or_else(|| format!("version not found: {}", version).into())
}