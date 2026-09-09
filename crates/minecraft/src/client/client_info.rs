use std::error::Error;
use std::fs;
use std::path::Path;

use crate::id::fetches;

pub async fn client_url(id: &str, path: &Path) -> Result<(), Box<dyn Error>> {
    let mani = fetches(id).await?;
    let id_final = &mani.url;
    println!("{}", id_final);

    let resp = reqwest::get(id_final).await?;
    let bytes = resp.bytes().await?;
    let dest = path.join(format!("{}.json", id));
    fs::write(&dest, &bytes)?;

    Ok(())
}

pub fn client_inf() {}
