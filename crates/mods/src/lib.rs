use clients::base_client;
use serde::{Serialize, Deserialize};

const API: &str = "https://api.modrinth.com/v2";

#[derive(Serialize, Deserialize)]
pub struct Hits {
     pub hits: Vec<ModDisplay>,
     pub total_hits: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ModDisplay {
     pub project_id: String,
     pub project_type: String,
     pub slug: String,
     pub title: String,
     pub categories: Vec<String>,
     pub versions: Vec<String>,
     pub icon_url: String,
     pub downloads: u64
}

pub async fn search_mod(query: &str, loader: &str, game: &str, offset: u64) -> Result<Vec<ModDisplay>, String> {
     let client = base_client();

     let url = if loader.is_empty() || game.is_empty() {
          format!("{}/search?query={}&limit=50&index=downloads&offset={}", API, query, offset)
     } else {
          let options = format!("[[\"categories:{}\"],[\"versions:{}\"],[\"project_type:mod\"]]", loader, game);
          format!("{}/search?query={}&facets={}&limit=50&index=downloads&offset={}", API, query, options, offset)
     };
     
     let resp = client
          .get(&url).send().await.map_err(|e| e.to_string())?
          .error_for_status().map_err(|e| e.to_string())?
          .json::<Hits>().await.map_err(|e| e.to_string())?;

     Ok(resp.hits)
}