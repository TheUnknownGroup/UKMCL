use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DirectoryConfig {
     pub game_dir: String,
     pub lib_dir: String,
     pub assets_dir: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstanceConfig {
     pub instance: String,
     pub minecraft_version: String,
     pub assets_index: String,
     pub main_class: String,
     pub directory: DirectoryConfig,
     pub classpath: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Main {
     pub main: InstanceConfig,
}

impl DirectoryConfig {
     pub fn inst(inst_dir: &str, lib_dir: &str, assets_dir: &str) -> Self {
          Self {
               game_dir: inst_dir.into(),
               lib_dir: lib_dir.into(),
               assets_dir: assets_dir.into(),
          }
     }
}

#[allow(unused)]
impl Main {
     pub fn new(inst_name: &str, id: &str, index: &str, main_class: &str, inst_dir: &str, lib_dir: &str, assets_dir: &str, classes: &[String], client_jar: &str) -> Result<Self, String> {
          Ok(Self {
               main: InstanceConfig{
                    instance: inst_name.into(),
                    minecraft_version: id.into(),
                    assets_index: index.into(),
                    main_class: main_class.into(),
                    directory: DirectoryConfig::inst(inst_dir, lib_dir, assets_dir),
                    classpath: Main::build_class(classes, client_jar).clone(),
               },
          })
     }

     pub fn build_class(lib_paths: &[String], client_jar: &str) -> Vec<String> {
          let mut all = lib_paths.to_vec();
          all.push(client_jar.into());
          all
     }

     pub fn save(&self, path: &str) -> std::io::Result<()> {
          let toml_str = toml::to_string_pretty(self).unwrap();
          std::fs::write(path, toml_str)
     }

     pub fn load(path: &str) -> std::io::Result<Self> {
          let data = std::fs::read_to_string(path)?;
          let config: Main = toml::from_str(&data).expect("Failed to parse");
          Ok(config)
     }
}