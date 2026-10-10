use serde::{Serialize, Deserialize};
use reqwest::Client;

const USER_AGENT: &str = "UKMCL/1.0.0 (kaylordevon5@gmail.com)";
const API: &str = "https://api.modrinth.com/v2";

#[derive(Deserialize, Serialize, Debug)]
pub struct Modrinth {
     pub loaders: Vec<String>,
     pub id: String,
     pub name: String,
     pub version_number: String,
     pub files: Vec<File>,
     pub dependencies: Option<Vec<Depends>>
}

#[derive(Deserialize, Debug, Serialize)]
pub struct File {
     pub url: String,
     pub filename: String,
     pub primary: bool,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Depends {
     pub version_id: Option<String>,
     pub project_id: String,
     pub file_name: Option<String>,
     pub dependency_type: String,
}

async fn list_version_for_game(project_id: &str, game: &str, loader: &str) -> Result<Vec<Modrinth>, String> {
     let client = Client::builder().user_agent(USER_AGENT).build().map_err(|e| e.to_string())?;

     let url = format!("{}/project/{}/version?game_versions=[\"{}\"]&loaders=[\"{}\"]", API, project_id, game, loader);
     let resp: Vec<Modrinth> = client.get(url).send().await.map_err(|e| e.to_string())?.error_for_status().map_err(|e| e.to_string())?.json().await.map_err(|e| e.to_string())?;

     Ok(resp)
}

pub async fn download(url: &str, file: &str, inst_name: &str) -> Result<(), String> {
     let client = Client::builder().user_agent(USER_AGENT).build().map_err(|e| e.to_string())?;

     let home = folders::hub_fold::make_hub().map_err(|e| e.to_string())?;
     let mods = home.join("instances").join(inst_name).join("minecraft").join("mods");
     if !mods.exists() {
          tokio::fs::create_dir_all(&mods).await.map_err(|e| e.to_string())?;
     }
     let bytes = client.get(url).send().await.map_err(|e| e.to_string())?.error_for_status().map_err(|e| e.to_string())?.bytes().await.map_err(|e| e.to_string())?;
     let dest = mods.join(file);

     if dest.exists() { return Ok(()); };
     
     tokio::fs::write(&dest, &bytes).await.map_err(|e| e.to_string())
}

pub async fn download_deps(version_id: Option<&str>, project_id: &str, game: &str, loader: &str, inst_name: &str) -> Result<(), String> {
     let info = list_version_for_game(project_id, game, loader).await?;
     let ver = match version_id {
          Some(id) => info.iter().find(|v| v.id == id),
          None => info.first(),
     }.ok_or_else(|| format!("version not found for {}", project_id))?;
     let file = ver.files.iter().find(|v| v.primary).or_else(|| ver.files.first()).ok_or("no files found")?;
     
     download(&file.url, &file.filename, inst_name).await.map_err(|e| e.to_string())
}

pub async fn save(project_id: &str, game: &str, loader: &str) -> Result<(), String> {
     let home = folders::hub_fold::make_hub().map_err(|e| e.to_string())?;
     let confg = home.join("jsons").join(format!("{}-{}.json", project_id, game));
     let data = list_version_for_game(project_id, game, loader).await?;
     let json = serde_json::to_string_pretty(&data).map_err(|e| e.to_string())?;
     tokio::fs::write(&confg, json).await.map_err(|e| e.to_string())
}

pub async fn load(project_id: &str, game: &str) -> Result<Vec<Modrinth>, String> {
     let home = folders::hub_fold::make_hub().map_err(|e| e.to_string())?;
     let cg = &home.join("jsons").join(format!("{}-{}.json", project_id, game));
     let file = std::fs::read_to_string(cg).map_err(|e| e.to_string())?;
     let info: Vec<Modrinth> = serde_json::from_str(&file).map_err(|e| e.to_string())?;

     Ok(info)
}