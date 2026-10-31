use std::{collections::HashSet, path::Path};
use anyhow::{Error, Result, Context};
use futures_util::{stream, StreamExt, TryStreamExt};
use utils::client::base;
use crate::minecraft::{CommonLibs, Libraries, Rule, download::download_to};

async fn download_all(root: &Path, mut jobs: Vec<(String, String)>) -> Result<Vec<String>> {
     let mut seen = HashSet::new();
     jobs.retain(|(rel, _)| seen.insert(rel.clone()));
     let client = base();
     stream::iter(jobs)
          .map(|(rel, url)| {
               let client = client.clone();
               let dest = root.join(rel);
               async move {
                    download_to(&client, &url, &dest, None).await?;
                    Ok::<_, Error>(dest.to_string_lossy().into_owned())
               }
          }).buffer_unordered(10).try_collect().await
}

pub async fn fetch_libraries(libs: &[Libraries], main: &Path) -> Result<Vec<String>> {
     let jobs = libs.iter()
          .filter(|s| applies(&s.rules))
          .filter_map(|l| l.downloads.artifact.as_ref())
          .map(|a| (a.path.clone(), a.url.clone()))
          .collect();
     download_all(&main.join("libraries"), jobs).await
}

pub async fn fabric_libraries(libs: &[CommonLibs], loader_version: &str, main: &Path, intermediary: &str, vanilla_libraries: &[Libraries], loader: &str) -> Result<Vec<String>> {
     let base = libs.first().map(|v| v.url.as_str()).context("no libs")?;
     let mut jobs = Vec::new();

     if loader == "fabric" {
          jobs.push((names(intermediary), urls(base, intermediary)));
          jobs.push((names(loader_version), urls(base, loader_version)));
     } else if loader == "quilt" {
          jobs.push((names(loader_version), urls("https://maven.quiltmc.org/repository/release/", loader_version)));
     }
     for lib in libs {
          jobs.push((names(&lib.name), urls(&lib.url, &lib.name)));
     }
     for lib in vanilla_libraries.iter().filter(|l| applies(&l.rules)) {
          if let Some(a) = &lib.downloads.artifact {
               jobs.push((a.path.clone(), a.url.clone()));
          }
     }
     download_all(&main.join("libraries"), jobs).await
}

pub fn current_os() -> &'static str {
     if cfg!(target_os = "windows") { "windows" }
     else if cfg!(target_os = "macos") { "osx" }
     else { "linux" }
}

pub fn applies(rules: &Option<Vec<Rule>>) -> bool {
     match rules {
          None => true,
          Some(rs) => rs.iter().all(|r| {
               let os_match = r.os.as_ref()
                    .and_then(|o| o.name.as_deref())
                    .map(|n| n == current_os())
                    .unwrap_or(true);
               match r.action.as_str() {
                    "allow" => os_match || r.os.is_none(),
                    "disallow" => !os_match,
                    _ => true,
               }
          })
     }
}

fn urls(base: &str, name: &str) -> String {
     let base = base.trim_end_matches('/');
     let mut it = name.split(":");
     let group = it.next().unwrap_or("");
     let artifact = it.next().unwrap_or("");
     let version = it.next().unwrap_or("");
     format!("{base}/{}/{artifact}/{version}/{artifact}-{version}.jar", group.replace(".", "/"))
}

fn names(name: &str) -> String {
     let mut it = name.split(":");
     let group = it.next().unwrap_or("");
     let artifact = it.next().unwrap_or("");
     let version = it.next().unwrap_or("");
     format!("{}/{artifact}/{version}/{artifact}-{version}.jar", group.replace(".", "/"))
}