pub mod library;
pub mod download;
pub mod assets;
pub mod version;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientInfo {
     pub main_class: String,
     pub asset_index: AssetIn,
     pub downloads: Downloads,
     pub libraries: Vec<Libraries>,
     pub java_version: JavaVersion,
     pub id: String,
     pub arguments: Option<Game>,
}

#[derive(Deserialize, Debug)]
pub struct Game {
     pub game: Vec<Arg>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Arg {
     Plain(String),
     Conditional { rules: Vec<Rules> },
}

#[derive(Deserialize, Debug)]
pub struct Rules {
     #[serde(default)]
     pub features: HashMap<String, bool>,
}

#[derive(Deserialize, Debug)]
pub struct AssetIn {
     pub id: String,
     pub url: String,
     pub size: Option<u64>,
     pub sha1: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Downloads {
     pub client: ClientJar,
}

#[derive(Deserialize, Debug)]
pub struct ClientJar {
     pub url: String,
     pub size: Option<u64>,
     pub sha1: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Libraries {
     pub name: String,
     pub downloads: Artifact,
     pub rules: Option<Vec<Rule>>,
}

#[derive(Deserialize, Debug)]
pub struct Rule {
     pub action: String,
     pub os: Option<Os>,
}

#[derive(Deserialize, Debug)]
pub struct Os {
     pub name: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct Artifact {
     pub artifact: Option<JarDownload>,
}

#[derive(Deserialize, Debug)]
pub struct JarDownload {
     pub path: String,
     pub url: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JavaVersion {
     pub component: String,
     pub major_version: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Fabric {
     pub loader: LoadInfo,
     pub launcher_meta: LauncherMeta,
     pub intermediary: Intermediary,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Quilt {
     pub loader: LoadInfo,
     pub launcher_meta: LauncherMeta,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadInfo {
     pub maven: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Intermediary {
     pub maven: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LauncherMeta {
     pub libraries: Libs,
     pub main_class: MainClass,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Libs {
     pub common: Vec<CommonLibs>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommonLibs {
     pub name: String,
     pub url: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MainClass {
     pub client: String,
}

impl ClientInfo {
     pub fn supports(&self, feature: &str) -> bool {
         self.arguments.as_ref().is_some_and(|a| {
             a.game.iter().any(|arg| match arg {
                 Arg::Conditional { rules } => rules.iter().any(|r| r.features.contains_key(feature)),
                 _ => false,
             })
         })
     }
}