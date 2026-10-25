use clients::base_client;
use serde::Deserialize;
use urlencoding::encode;

use crate::modrinth::{Hits, ModDisplay};

const API: &str = "https://api.curseforge.com/v1";
const MINECRAFT: u32 = 432;

pub const CURSEFORGE_API_KEY: &str = "$2a$10$f16D31uQ7HXzyJ77NqA.kexdtUsRVNACnjisbhRa31xWRDg1DA/CS";
// const CURSEFORGE_API_KEY: &str = match option_env!("CURSEFORGE_API_KEY") { Some(id) => id, None => "fallback-id" };

fn class_id(kind: &str) -> Result<u32, String> {
     match kind {
          "mods" => Ok(6),
          "modpacks" => Ok(4471),
          "shaders" => Ok(6552),
          other => Err(format!("unknown: {other}")),
     }
}

pub fn loader_type(l: &str) -> Option<u8> {
     match l {
          "forge" => Some(1),
          "fabric" => Some(4),
          "quilt" => Some(5),
          "neoforge" => Some(6),
          _ => None,
     }
}

#[derive(Deserialize)]
struct CurseForge {
     data: Vec<CfMod>,
     pagination: CfPage
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CfMod {
     id: u64, 
     name: String,
     slug: String,
     #[serde(default)]
     download_count: f64,
     logo: Option<CfLogo>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CfLogo {
     thumbnail_url: Option<String>,
     url: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CfPage {
     total_count: u64,
}

pub async fn cf_search(kind: &str, query: &str, loader: &str, game: &str, offset: u64) -> Result<Hits, String> {
     let mut url = format!("{API}/mods/search?gameId={MINECRAFT}&classId={}&sortField=6&sortOrder=desc&pageSize=50&index={offset}", class_id(kind)?);
     if !query.is_empty() { url.push_str(&format!("&searchFilter={}", encode(query))); }
     if !game.is_empty() { url.push_str(&format!("&gameVersion={}", encode(game))); }
     if let Some(t) = loader_type(loader) { url.push_str(&format!("&modLoaderType={t}")); }
     let resp = base_client().get(&url).header("x-api-key", CURSEFORGE_API_KEY).header("Accept", "application").send().await.map_err(|e| e.to_string())?
          .error_for_status().map_err(|e| e.to_string())?
          .json::<CurseForge>().await.map_err(|e| e.to_string())?;
     let hits = resp.data.into_iter().map(|m| ModDisplay {
          project_id: m.id.to_string(),
          slug: m.slug,
          title: m.name,
          icon_url: m.logo.and_then(|l| l.thumbnail_url.or(l.url)).unwrap_or_default(),
          downloads: m.download_count as u64,
     }).collect();

     Ok(Hits { hits, total_hits: resp.pagination.total_count.min(10_000) })
}