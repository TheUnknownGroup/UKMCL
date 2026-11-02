use std::{collections::HashMap, path::{Component, Path, PathBuf}, process::Command, sync::{Arc, OnceLock, atomic::{AtomicU64, Ordering::Relaxed}}};
use anyhow::{Context, Error, Result, bail};
use futures_util::{StreamExt, TryStreamExt, stream};
use regex::Regex;
use tauri::AppHandle;
use utils::{client::base, progress::emit_progress};
use crate::minecraft::{CommonLibs, Libraries, Rule, download::download_to_verified};

fn relative_path(value: &str) -> Result<PathBuf> {
     if value.is_empty() || value.contains('\\') || value.contains(':') || value.contains('\0') || value.chars().any(char::is_control) {
          bail!("invalid library path: {value:?}");
     }
     let path = Path::new(value);
     if path.is_absolute() || path.components().any(|c| !matches!(c, Component::Normal(_))) || value.split('/').any(|p| p.is_empty() || p == "." || p == "..") {
          bail!("library path mus be a clean relative path: {value:?}");
     }
     Ok(path.to_path_buf())
}

async fn download_all(app: &AppHandle, phase: &str, root: &Path, jobs: Vec<(String, String, Option<u64>, Option<String>)>) -> Result<Vec<String>> {
     let mut uni: Vec<(String, String, Option<u64>, Option<String>)> = Vec::new();
     let mut positions: HashMap<String, usize> = HashMap::new();
     for (relative, url, size, sha1) in jobs {
          relative_path(&relative)?;
          if let Some(index) = positions.get(&relative).copied() {
               let existing = &uni[index];
               if existing.1 != url || existing.2 != size || existing.3 != sha1 {
                    bail!("conflicting metadata for library artifact path {relative:?}");
               }
          } else {
               positions.insert(relative.clone(), uni.len());
               uni.push((relative, url, size, sha1));
          }
     }
     let file_total = uni.len() as u32;
     let all_sizes = uni.iter().all(|j| j.2.is_some());
     let total = if all_sizes {
          uni.iter().map(|j| j.2.unwrap_or(0)).sum()
     } else {
          0
     };
     let downloaded = Arc::new(AtomicU64::new(0));
     let files = Arc::new(AtomicU64::new(0));
     emit_progress(app, phase, 0, total, "Preparing Libraris", 0, file_total)?;
     
     let client = base();
     let results: Vec<String> = stream::iter(uni)
          .map(|(relative, url, size, sha1)| {
               let client = client.clone();
               let app = app.clone();
               let phase = phase.to_owned();
               let root = root.to_path_buf();
               let downloaded = downloaded.clone();
               let files = files.clone();
               async move {
                    let destination = root.join(&relative);
                    download_to_verified(&client, &url, &destination, size, sha1.as_deref()).await
                         .with_context(|| format!("downloaded library {relative}"))?;
                    let done_f = files.fetch_add(1, Relaxed) + 1;
                    let done_b = if all_sizes {
                         downloaded.fetch_add(size.unwrap_or(0), Relaxed) + size.unwrap_or(0)
                    } else { 0 };
                    emit_progress(&app, &phase, done_b, total, relative.clone(), done_f as u32, file_total)?;
                    Ok::<String, Error>(destination.to_string_lossy().into_owned())
               }
          }).buffered(10).try_collect().await?;
     emit_progress(app, phase, total, total, "Libraries Complete", file_total, file_total)?;
     Ok(results)
}

pub async fn fetch_libraries(app: &AppHandle, libs: &[Libraries], main: &Path) -> Result<Vec<String>> {
     let jobs = libs.iter()
          .filter(|s| applies(&s.rules))
          .filter_map(|l| l.downloads.artifact.as_ref())
          .map(|a| {
               (a.path.clone(), a.url.clone(), a.size, a.sha1.clone())
          })
          .collect();
     download_all(&app, "libraries", &main.join("libraries"), jobs).await
}

pub async fn fabric_libraries(app: &AppHandle, libs: &[CommonLibs], loader_version: &str, main: &Path, intermediary: &str, vanilla_libraries: &[Libraries], loader: &str) -> Result<Vec<String>> {
     let base = libs.first().map(|v| v.url.as_str()).context("no libs")?;
     let mut jobs = Vec::new();

     if loader == "fabric" {
          jobs.push((names(intermediary), urls(base, intermediary), None, None));
          jobs.push((names(loader_version), urls(base, loader_version), None, None));
     } else if loader == "quilt" {
          jobs.push((names(loader_version), urls("https://maven.quiltmc.org/repository/release/", loader_version), None, None));
     }
     for lib in libs {
          jobs.push((names(&lib.name), urls(&lib.url, &lib.name), None, None));
     }
     for lib in vanilla_libraries.iter().filter(|l| applies(&l.rules)) {
          if let Some(a) = &lib.downloads.artifact {
               jobs.push((a.path.clone(), a.url.clone(), a.size, a.sha1.clone()));
          }
     }
     download_all(&app, "libraries", &main.join("libraries"), jobs).await
}

pub fn current_os() -> &'static str {
     if cfg!(target_os = "windows") { "windows" }
     else if cfg!(target_os = "macos") { "osx" }
     else { "linux" }
}

fn current_arch() -> &'static str {
     match std::env::consts::ARCH {
          "x86_64" => "x86_64",
          "x86" | "i386" | "i586" | "i686" => "x86",
          "aarch64" => "aarch64",
          other => other,
     }
}

fn current_os_ver() -> Option<String> {
     #[cfg(target_os = "windows")]
     let out = Command::new("cmd").args(["/C", "ver"]).output().ok()?;
     #[cfg(target_os = "macos")]
     let out = Command::new("sw_vers").arg("-productVersion").output().ok()?;
     #[cfg(target_os = "linux")]
     let out = Command::new("uname").arg("-r").output().ok()?;
     #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
     {
          return String::from_utf8(out.stdout).ok().map(|v| v.trim().to_owned());
     }
     #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
     None
}

fn rule_matches(rule: &Rule, os_name: &str, arch: &str, os_version: Option<&str>) -> bool {
    let Some(os) = &rule.os else {
        return true;
    };

    if os.name.as_deref().is_some_and(|name| name != os_name) {
        return false;
    }
    if os.arch.as_deref().is_some_and(|expected| expected != arch) {
        return false;
    }
    if let Some(pattern) = os.version.as_deref() {
        let Some(version) = os_version else {
            return false;
        };
        let Ok(regex) = Regex::new(pattern) else {
            return false;
        };
        if !regex.is_match(version) {
            return false;
        }
    }

    true
}

pub fn applies(rules: &Option<Vec<Rule>>) -> bool {
     let Some(rules) = rules else {
          return true;
     };
     static OS_VERSION: OnceLock<Option<String>> = OnceLock::new();
     let version = OS_VERSION.get_or_init(current_os_ver);
     let mut allow = false;
     for rule in rules {
          if rule_matches(rule, current_os(), current_arch(), version.as_deref()) {
               match rule.action.as_str() {
                    "allow" => allow = true,
                    "disallow" => allow = false,
                    _ => {}
               }
          }
     }
     allow
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