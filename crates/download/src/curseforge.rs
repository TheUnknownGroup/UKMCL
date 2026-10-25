use std::{collections::{HashMap, HashSet}, io::Read, path::{Path, PathBuf} };
use clients::base_client;
use create::insts::setup_inst;
use futures::stream::{self, StreamExt};
use serde::{Deserialize, Serialize};
use store::curseforge::{loader_type, CURSEFORGE_API_KEY};
use tauri::AppHandle;

const API: &str = "https://api.curseforge.com/v1";
const RELEASE: u8 = 1;
const REQUIRED: u8 = 3;

#[derive(Deserialize)]
struct Data<T> { 
     data: T 
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct CfFile {
     id: u64,
     mod_id: u64,
     file_name: String,
     download_url: Option<String>,
     #[serde(default)] 
     release_type: u8,
     #[serde(default)] 
     file_date: String,
     #[serde(default)] 
     dependencies: Vec<CfDep>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct CfDep {
     mod_id: u64, 
     relation_type: u8,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CfMod {
     id: u64,
     name: String,
     class_id: Option<u32>,
     links: Option<CfLinks>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CfLinks {
     website_url: Option<String>
}

#[derive(Serialize, Clone)]
pub struct Skipped {
     pub name: String,
     pub page: String,
}

async fn get<T: serde::de::DeserializeOwned>(path: &str) -> Result<T, String> {
     base_client().get(format!("{API}{path}"))
          .header("x-api-key", CURSEFORGE_API_KEY).send()
          .await.map_err(|e| e.to_string())?.error_for_status()
          .map_err(|e| e.to_string())?.json::<T>().await.map_err(|e| e.to_string())
}

async fn post<T: serde::de::DeserializeOwned>(path: &str, body: serde_json::Value) -> Result<T, String> {
     base_client().post(format!("{API}{path}"))
          .header("x-api-key", CURSEFORGE_API_KEY).json(&body).send()
          .await.map_err(|e| e.to_string())?
          .error_for_status().map_err(|e| e.to_string())?
          .json::<T>().await.map_err(|e| e.to_string())
}

async fn latest_file(mod_id: u64, game: &str, loader: &str) -> Result<CfFile, String> {
     let mut path = format!("/mods/{mod_id}/files?pageSize=50");
     if !game.is_empty() { path.push_str(&format!("&gameVersion={game}")); }
     if let Some(t) = loader_type(loader) { path.push_str(&format!("&modLoaderType={t}")); }
     let mut files = get::<Data<Vec<CfFile>>>(&path).await?.data;
     files.sort_by(|a, b| b.file_date.cmp(&a.file_date));
     files.iter().find(|f| f.release_type == RELEASE).or_else(|| files.first()).cloned().ok_or_else(|| format!("no curseforge file found for {mod_id} on {game}"))
}

async fn url_for(f: &CfFile) -> Option<String> {
     if let Some(u) = f.download_url.as_ref().filter(|u| !u.is_empty()) {
          return Some(u.clone());
     }
     get::<Data<String>>(&format!("/mods/{}/files/{}/download-url", f.mod_id, f.id))
          .await.ok().map(|d| d.data).filter(|u| !u.is_empty())
}

fn safe_name(n: &str) -> Result<&std::ffi::OsStr, String> {
     Path::new(n).file_name().ok_or_else(|| format!("bad file name found: {n}"))
}

fn describe(projects: &HashMap<u64, CfMod>, id: u64) -> Skipped {
     match projects.get(&id) {
          Some(p) => Skipped {
               name: p.name.clone(),
               page: p.links.as_ref().and_then(|l| l.website_url.clone()).unwrap_or_default(),
          },
          None => Skipped { name: format!("project {id}"), page: String::new() },
     }
}

async fn describe_remote(id: u64) -> Skipped {
     let mut one = HashMap::new();
     if let Ok(r) = get::<Data<CfMod>>(&format!("/mods/{id}/")).await { one.insert(id, r.data); }
     describe(&one, id)
}

pub async fn install_project(kind: &str, project_id: u64, game: &str, loader: &str, inst_name: &str) -> Result<Vec<Skipped>, String> {
     let home = folders::hub_fold::make_hub().map_err(|e| e.to_string())?;
     let (sub, loader) = match kind {
          "mods" => ("mods", loader),
          "shaders" => ("shaderpacks", ""),
          other => return Err(format!("unknown kind: {other}")),
     };
     let dir = home.join("instances").join(inst_name).join("minecraft").join(sub);
     if !dir.exists() {
          tokio::fs::create_dir_all(&dir).await.map_err(|e| e.to_string())?;
     }

     let mut skipped = Vec::new();
     let mut seen = HashSet::new();
     let mut queue = vec![project_id];
     while let Some(id) = queue.pop() {
          if !seen.insert(id) { continue; }
          let file = match latest_file(id, game, loader).await {
               Ok(f) => f,
               Err(e) if id == project_id => return Err(e),
               Err(_) => { skipped.push(describe_remote(id).await); continue; }
          };

          let dest = dir.join(safe_name(&file.file_name)?);
          if !dest.exists() {
               match url_for(&file).await {
                    Some(url) => crate::fetch_to(&base_client(), &url, &dest).await?,
                    None => skipped.push(describe_remote(file.mod_id).await),
               }
          }

          if kind == "mods" {
               queue.extend(file.dependencies.iter().filter(|d| d.relation_type == REQUIRED).map(|d| d.mod_id));
          }
     }
     Ok(skipped)
}

#[derive(Deserialize)]
struct Mani {
     name: String,
     minecraft: Minecraft,
     #[serde(default)] 
     files: Vec<ManiFile>,
     #[serde(default = "default_overrides")] 
     overrides: String,
}

fn default_overrides() -> String { "overrides".into() }

#[derive(Deserialize)]
struct Minecraft {
     version: String,
     #[serde(rename = "modLoaders", default)] 
     mod_loaders: Vec<ModLoader>,
}

#[derive(Deserialize)]
struct ModLoader { 
     id: String, 
     #[serde(default)] 
     primary: bool
}

#[derive(Deserialize)]
struct ManiFile {
     #[serde(rename = "projectID")]
     project_id: u64,
     #[serde(rename = "fileID")]
     file_id: u64,
     #[serde(default = "yes")]
     required: bool,
}

fn yes() -> bool { true }

pub async fn install_pack(app: AppHandle, project_id: u64, game: &str, loader: &str) -> Result<Vec<Skipped>, String> {
     let home = folders::hub_fold::make_hub().map_err(|e| e.to_string())?;
     let temp = home.join("temp");
     if !temp.exists() {
          tokio::fs::create_dir_all(&temp).await.map_err(|e| e.to_string())?;
     }
     let pack = latest_file(project_id, game, loader).await?;
     let url = url_for(&pack).await.ok_or("this pack's author doesn't allow downloads")?;
     let zip_path = temp.join(safe_name(&pack.file_name)?);
     if !zip_path.exists() { crate::fetch_to(&base_client(), &url, &zip_path).await?; }
     let file = std::fs::File::open(&zip_path).map_err(|e| e.to_string())?;
     let mut arch = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;
     let mut text = String::new();
     arch.by_name("manifest.json").map_err(|e| e.to_string())?.read_to_string(&mut text).map_err(|e| e.to_string())?;
     let manifest: Mani = serde_json::from_str(text.trim_start_matches('\u{feff}')).map_err(|e| e.to_string())?;

     let main = manifest.minecraft.mod_loaders.iter().find(|l| l.primary)
          .or(manifest.minecraft.mod_loaders.first()).ok_or("the manifest doesn't list a mod loader")?;
     let (loader_name, loader_ver) = main.id.split_once('-').ok_or("unrecognized loader")?;
     if !matches!(loader_name, "fabric" | " quilt" | "neoforge") {
          return Err(format!("{loader_name} packs are supported"));
     }

     let mc = manifest.minecraft.version.clone();
     let inst_dir = setup_inst(app, format!("{} - {}", manifest.name, mc), mc, loader_name.to_string(), loader_ver.to_string())
          .await.map_err(|e| e.to_string())?;
     let result = fill_pack(&manifest, &mut arch, &inst_dir.join("minecraft")).await;
     let _ = tokio::fs::remove_dir_all(&temp).await;
     if result.is_err() { let _ = tokio::fs::remove_dir_all(&inst_dir).await; }
     result
}

async fn fill_pack(mani: &Mani, arch: &mut zip::ZipArchive<std::fs::File>, inst: &Path) -> Result<Vec<Skipped>, String> {
     let wanted: Vec<&ManiFile> = mani.files.iter().filter(|f| f.required).collect();
     let mut files: HashMap<u64, CfFile> = HashMap::new();
     for chunk in wanted.chunks(50) {
          let ids: Vec<u64> = chunk.iter().map(|f| f.file_id).collect();
          let r: Data<Vec<CfFile>> = post("/mods/files", serde_json::json!({ "fileIds": ids })).await?;
          files.extend(r.data.into_iter().map(|f| (f.id, f)));
     }

     let mut projects: HashMap<u64, CfMod> = HashMap::new();
     let ids: Vec<u64> = wanted.iter().map(|f| f.project_id).collect::<HashSet<_>>().into_iter().collect();
     for chunk in ids.chunks(50) {
          let r: Data<Vec<CfMod>> = post("/mods", serde_json::json!({ "modIds": chunk, "filterPcOnly": true })).await?;
          projects.extend(r.data.into_iter().map(|m| (m.id, m)));
     }
     let mut todo: Vec<(String, PathBuf)> = Vec::new();
     let mut skipped: Vec<Skipped> = Vec::new();
     for mf in &wanted {
          let Some(file) = files.get(&mf.file_id) else {
               skipped.push(describe(&projects, mf.project_id));
               continue;
          };
          let sub = match projects.get(&mf.project_id).and_then(|p| p.class_id) {
               Some(12) => "resourcepacks",
               Some(6552) => "shaderpacks",
               _ => "mods",
          };
          let dir = inst.join(sub);
          if !dir.exists() {
               tokio::fs::create_dir_all(&dir).await.map_err(|e| e.to_string())?;
          }
          let dest = dir.join(safe_name(&file.file_name)?);
          if dest.exists() { continue; }
          match url_for(file).await {
               Some(url) => todo.push((url, dest)),
               None => skipped.push(describe(&projects, mf.project_id)),
          }
     }
     let client = base_client();
     let errors: Vec<String> = stream::iter(todo)
          .map(|(url, dest)| {
               let client = client.clone();
               async move {
                    crate::fetch_to(&client, &url, &dest).await.map_err(|e| format!("{}: {e}", dest.display()))
               }
          }).buffer_unordered(6).filter_map(|r| async move { r.err() }).collect().await;

     if !errors.is_empty() {
          return Err(format!("{} files failed to download: {}", errors.len(), errors.join("; ")));
     }

     crate::overrides(arch, &mani.overrides, inst)?;
     Ok(skipped)
}