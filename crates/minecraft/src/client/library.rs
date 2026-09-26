use std::path::{PathBuf};
use tokio::io::AsyncWriteExt;
use futures_util::StreamExt;

use crate::client::{Libraries, CommonLibs, applies};

pub async fn fetch_libraries(libs: &[Libraries], main: &PathBuf) -> Result<Vec<String>, Box<dyn std::error::Error>>{
     let mut class = Vec::new();
     let path = main.join("libraries");
     
     for lib in libs {
          if !applies(&lib.rules) {
               continue;
          }

          if let Some(artifact) = &lib.downloads.artifact {
               let dest = destination(&path, &artifact.path, &artifact.url).await?;
               class.push(dest.to_string_lossy().to_string());
          }
     }
     
     Ok(class)
}

pub async fn fabric_libraries(libs: &[CommonLibs], loader: &str, main: &PathBuf, inter: &str, vanilla_libs: &[Libraries], loaders: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
     let mut class = Vec::new();
     let path = main.join("libraries");

     let url = libs.first().map(|v| v.url.as_str()).ok_or("no libs")?;
     let loader_down = urls(url, loader);     
     let loader_path = names(loader);
     let quilt_down = urls("https://maven.quiltmc.org/repository/release/", loader);

     let inter_down = urls(url, inter);
     let inter_path = names(inter);

     if loaders == "fabric" {
          let dest1 = destination(&path, &inter_path, &inter_down).await?;
          class.push(dest1.to_string_lossy().to_string());
          let dest2 = destination(&path, &loader_path, &loader_down).await?;
          class.push(dest2.to_string_lossy().to_string());
     } else if loaders == "quilt" {
          let dest = destination(&path, &loader_path, &quilt_down).await?;
          class.push(dest.to_string_lossy().to_string());
     }

     for lib in libs {
          let name = names(&lib.name);
          let url = urls(&lib.url, &lib.name);

          let dest = destination(&path, &name, &url).await?;
          class.push(dest.to_string_lossy().to_string());
     }

     for lib in vanilla_libs {
          if !applies(&lib.rules) {
               continue;
          }

          if let Some(artifact) = &lib.downloads.artifact {
               let dest = destination(&path, &artifact.path, &artifact.url).await?;
               class.push(dest.to_string_lossy().to_string());
          }
     }

     Ok(class)
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

async fn destination(path: &PathBuf, paths: &String, url: &str) -> anyhow::Result<PathBuf> {
     let dest = path.join(paths);

     if !dest.exists() {
          std::fs::create_dir_all(dest.parent().unwrap())?;
          let bys = reqwest::get(url).await?;
          let mut stream = bys.bytes_stream();
          let mut file = tokio::fs::File::create(&dest).await?;
          while let Some(chunk) = stream.next().await {
               let chunk = chunk?;
               file.write_all(&chunk).await?;
          }
     }

     Ok(dest)
}