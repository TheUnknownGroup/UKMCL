use serde::Deserialize;
use reqwest::Result;
use utils::client::base;

const NEOFORGE_URL: &str = "https://maven.neoforged.net/api/maven/versions/releases/net/neoforged";

#[derive(Deserialize, Debug)]
pub struct NeoForge {
     pub versions: Vec<String>,
}

async fn fetch_neoforge_versions(client: &reqwest::Client, art: &str) -> Result<Vec<String>> {
     let resp: NeoForge = client.get(format!("{NEOFORGE_URL}/{art}")).send().await?.error_for_status()?.json().await?;
     Ok(resp.versions)
}

pub async fn neo() -> Result<Vec<String>> {
     let client = base();
     let mut versions = fetch_neoforge_versions(&client, "neoforge").await?;
     if let Ok(legacy) = fetch_neoforge_versions(&client, "forge").await {
          versions.extend(legacy);
     }
     versions.sort_by_key(|v| std::cmp::Reverse(sort_key(v)));
     versions.dedup();
     Ok(versions)
}

fn sort_key(version: &str) -> (bool, Vec<u32>) {
     let tail = if version.starts_with("1.") {
          version.split_once('-').map_or(version, |(_, rest)| rest)
     } else {
          version
     };
     let is_release = !tail.contains('-') && !tail.contains('+');
     let core = tail.split(['-', '+']).next().unwrap_or_default();
     (is_release, nums(core).unwrap_or_default())
}

fn nums(val: &str) -> Option<Vec<u32>> {
     val.split('.').map(|p| p.parse().ok()).collect()
}

pub fn mc_ver(version: &str) -> Option<String> {
     if version.starts_with("1.") {
          let (mine, _) = version.split_once('-')?;
          nums(mine)?;
          return Some(mine.to_owned());
     }
     let core = version.split(['-', '+']).next()?;
     let numbers = nums(core)?;
     if numbers.len() < 3 {
          return None;
     }

     if numbers[0] >= 26 {
          let mut minecraft = numbers[..numbers.len() - 1].to_vec();
          while minecraft.len() > 2 && minecraft.last() == Some(&0) {
               minecraft.pop();
          }
          Some(minecraft.iter().map(u32::to_string).collect::<Vec<_>>().join("."))
     } else if numbers[1] == 0 {
          Some(format!("1.{}", numbers[0]))
     } else {
          Some(format!("1.{}.{}", numbers[0], numbers[1]))
     }
}

pub fn for_mc(all: &[String], minecraft: &str) -> Vec<String> {
     let mut versions: Vec<&String> = all.iter().filter(|v| mc_ver(v).as_deref() == Some(minecraft)).collect();
     versions.sort_by_key(|v| std::cmp::Reverse(sort_key(v)));
     versions.into_iter().cloned().collect()
}

pub fn is_legacy(ver: &str) -> bool {
     ver.starts_with("1.")
}