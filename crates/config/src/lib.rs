pub mod write;
pub mod main_conf;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct MainBuilder {
     instance: String,
     minecraft_version: String,
     assets_index: String,
     main_class: String,
     game_dir: String,
     lib_dir: String,
     assets_dir: String,
     classpath: Vec<String>,
     loader: String,
     version: String,
     max: u32,
     min: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Main {
     pub main: InstanceConfig,
     pub loader: Option<LoaderConfig>,
     pub directory: DirectoryConfig,
     pub java: JavaConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoaderConfig {
     pub loader: String,
     pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstanceConfig {
     pub instance: String,
     pub minecraft_version: String,
     pub assets_index: String,
     pub main_class: String,
     pub classpath: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DirectoryConfig {
     pub game_dir: String,
     pub lib_dir: String,
     pub assets_dir: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JavaConfig {
     pub max: u32,
     pub min: u32,
}

impl Main {
     pub fn save(&self, path: &str) -> std::io::Result<()> {
          let toml_str = toml::to_string_pretty(self).unwrap();
          std::fs::write(path, toml_str)
     }
     
     pub fn load(path: &str) -> std::io::Result<Self> {
          let data = std::fs::read_to_string(path)?;
          let config: Self = toml::from_str(&data).expect("Failed to parse");
          Ok(config)
     }
}

impl MainBuilder {
     pub fn new(inst_name: &str, id: &str) -> Self {
          Self {
               instance: inst_name.into(),
               minecraft_version: id.into(),
               assets_index: String::new(),
               main_class: String::new(),
               game_dir: String::new(),
               lib_dir: String::new(),
               assets_dir: String::new(),
               classpath: Vec::new(),
               loader: String::new(),
               version: String::new(),
               max: 32768,
               min: 512,
          }
     }

     pub fn ver(mut self, ind: &str, class: &str, classpath: &[String], client_jar: &str) -> Self {
          self.assets_index = ind.into();
          self.main_class = class.into();
          self.classpath = MainBuilder::build_with_jar(classpath, client_jar);
          self
     }

     pub fn build_with_jar(classes: &[String], client_jar: &str) -> Vec<String> {
          let mut all = classes.to_vec();
          all.push(client_jar.into());
          all
     }

     pub fn directory(mut self, inst_dir: &str, lib_dir: &str, assets_dir: &str) -> Self {
          self.game_dir = inst_dir.into();
          self.lib_dir = lib_dir.into();
          self.assets_dir = assets_dir.into();
          self
     }

     pub fn java(mut self, max: u32, min: u32) -> Self {
          self.max = max.into();
          self.min = min.into();
          self
     }

     pub fn loaders(mut self, load: &str, ver: &str) -> Self {
          self.loader = load.into();
          self.version = ver.into();
          self
     }

     pub fn build(self) -> Main {
          Main {
               main: InstanceConfig {
                    instance: self.instance,
                    minecraft_version: self.minecraft_version,
                    assets_index: self.assets_index,
                    main_class: self.main_class,
                    classpath: self.classpath,
               },
               loader: Some(LoaderConfig {
                    loader: self.loader,
                    version: self.version
               }),
               directory: DirectoryConfig {
                    game_dir: self.game_dir,
                    lib_dir: self.lib_dir,
                    assets_dir: self.assets_dir,
               },
               java: JavaConfig {
                    max: self.max,
                    min: self.min,
               }
          }
     }

     pub fn save(self, path: &str) -> std::io::Result<Main> {
          let cfg = self.build();
          cfg.save(path)?;
          Ok(cfg)
     }
}