use crate::version_get::fetch_version;
use std::fs;
use std::path::Path;

pub async fn fetch_client(id: &str, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
     let mani = fetch_version(id).await?;
     let id_final = &mani.url;
     println!("{}", id_final);

     let respon = reqwest::get(id_final).await?;
     let bytes = respon.bytes().await?;
     let dest = path.join(format!("{}.json", id));
     fs::write(&dest, &bytes)?;
     
     Ok(())
}