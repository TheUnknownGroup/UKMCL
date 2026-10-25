use clients::base_client;
use serde::{Serialize, Deserialize};
use urlencoding::encode;

const API: &str = "https://api.modrinth.com/v2";

#[derive(Serialize, Deserialize)]
pub struct Hits {
     pub hits: Vec<ModDisplay>,
     pub total_hits: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ModDisplay {
     pub project_id: String,
     pub slug: String,
     pub title: String,
     pub icon_url: String,
     pub downloads: u64
}

pub async fn modrin_mod(query: &str, loader: &str, game: &str, offset: u64) -> Result<Hits, String> {
     let client = base_client();

     let url = if loader.is_empty() || game.is_empty() {
          format!("{}/search?query={}&limit=50&index=downloads&offset={}", API, encode(query), offset)
     } else {
          let options = format!("[[\"categories:{}\"],[\"versions:{}\"],[\"project_type:mod\"]]", loader, game);
          format!("{}/search?query={}&facets={}&limit=50&index=downloads&offset={}", API, encode(query), options, offset)
     };
     
     let resp = client
          .get(&url).send().await.map_err(|e| e.to_string())?
          .error_for_status().map_err(|e| e.to_string())?
          .json::<Hits>().await.map_err(|e| e.to_string())?;

     Ok(resp)
}

pub async fn modrin_p(query: &str, loader: &str, game: &str, offset: u64) -> Result<Hits, String> {
     let client = base_client();
     let mut opts: Vec<String> = vec![];
     if !game.is_empty() { opts.push(format!("[\"versions:{}\"]", game)); }
     if !loader.is_empty() { opts.push(format!("[\"categories:{}\"]", loader)); }
     opts.push("[\"project_type:modpack\"]".to_string());
     let param = format!("[{}]", opts.join(","));

     let url = format!("{}/search?query={}&limit=50&index=downloads&offset={}&facets={}", API, encode(query), offset, param);
     
     let resp = client
          .get(&url).send().await.map_err(|e| e.to_string())?
          .error_for_status().map_err(|e| e.to_string())?
          .json::<Hits>().await.map_err(|e| e.to_string())?;

     Ok(resp)
}

pub async fn modrin_s(query: &str, loader: &str, game: &str, offset: u64) -> Result<Hits, String> {
     let client = base_client();

     let mut opts: Vec<String> = vec![];
     if !game.is_empty() { opts.push(format!("[\"versions:{}\"]", game)); }
     if !loader.is_empty() { opts.push(format!("[\"categories:{}\"]", loader)); }
     opts.push("[\"project_type:shader\"]".to_string());
     let param = format!("[{}]", opts.join(","));
     
     let url = format!("{}/search?query={}&limit=50&index=downloads&offset={}&facets={}", API, encode(query), offset, param);
     
     let resp = client
          .get(&url).send().await.map_err(|e| e.to_string())?
          .error_for_status().map_err(|e| e.to_string())?
          .json::<Hits>().await.map_err(|e| e.to_string())?;

     Ok(resp)
}