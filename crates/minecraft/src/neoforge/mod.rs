pub mod install;

use clients::base_client;
use serde::{Deserialize};
use reqwest::Result;
const NEOFORGE_URL: &str = "https://maven.neoforged.net/api/maven/versions/releases/net/neoforged";

#[derive(Deserialize, Debug, Clone)]
pub struct NeoForge {
     pub versions: Vec<String>,
}

async fn fetc(client: &reqwest::Client, art: &str) -> Result<Vec<String>> {
     let resp: NeoForge = client.get(format!("{NEOFORGE_URL}/{art}")).send()
          .await?
          .json()
          .await?;
     Ok(resp.versions)
}

pub async fn neo() -> Result<Vec<String>> {
     let client = base_client();
     let mut all = fetc(&client, "neoforge").await?;
     if let Ok(leg) = fetc(&client, "forge").await {
          all.extend(leg);
     }
     Ok(all)
}

fn nums(s: &str) -> Option<Vec<u32>> {
     s.split('.').map(|p| p.parse().ok()).collect()
}

pub fn mc_ver(v: &str) -> Option<String> {
     if v.starts_with("1.") {
          let (mc, _) = v.split_once('-')?;
          nums(mc)?;
          return Some(mc.to_owned());
     }
     let core = v.split(['-', '+']).next()?;
     let n = nums(core)?;
     if n.len() < 3 { return None; }
     if n[0] >= 26 {
          let mut mc = n[..n.len() - 1].to_vec();
          while mc.len() > 2 && mc.last() == Some(&0) { mc.pop(); }
          Some(mc.iter().map(u32::to_string).collect::<Vec<_>>().join("."))
     } else if n[1] == 0 {
          Some(format!("1.{}", n[0]))
     } else {
          Some(format!("1.{}.{}", n[0], n[1]))
     }
}

fn sort(v: &str) -> (bool, Vec<u32>) {
     let tail = if v.starts_with("1.") { v.split_once('-').map_or(v, |x| x.1) } else { v };
     let pre = tail.contains('-') || tail.contains('+');
     let core = tail.split(['-', '+']).next().unwrap_or("");
     (!pre, nums(core).unwrap_or_default())
}

pub fn for_mc(all: &[String], mc: &str) -> Vec<String> {
     let mut v: Vec<&String> = all.iter().filter(|x| mc_ver(x).as_deref() == Some(mc)).collect();
     v.sort_by_key(|x| std::cmp::Reverse(sort(x)));
     v.into_iter().cloned().collect()
}

pub fn is_legacy(v: &str) -> bool {
     v.starts_with("1.")
}