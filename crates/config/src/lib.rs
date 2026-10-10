pub mod write;
pub mod main_conf;
pub mod writes;
pub mod msa;
pub mod offline;

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
     component: String,
     supports_quick_play_single: bool,
     supports_quick_play_multi: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Main {
     pub main: InstanceConfig,
     pub loader: Option<LoaderConfig>,
     pub directory: DirectoryConfig,
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
     pub component: String,
     pub supports_quick_play_single: bool,
     pub supports_quick_play_multi: bool,
     pub classpath: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DirectoryConfig {
     pub game_dir: String,
     pub lib_dir: String,
     pub assets_dir: String,
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
               component: String::new(),
               supports_quick_play_multi: false,
               supports_quick_play_single: false,
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

     pub fn loaders(mut self, load: &str, ver: &str) -> Self {
          self.loader = load.into();
          self.version = ver.into();
          self
     }

     pub fn javas(mut self, component: &str) -> Self {
          self.component = component.into();
          self
     }

     pub fn supports(mut self, multi: bool, single: bool) -> Self {
          self.supports_quick_play_multi = multi;
          self.supports_quick_play_single = single;
          self
     }

     pub fn build(self) -> Main {
          Main {
               main: InstanceConfig {
                    instance: self.instance,
                    minecraft_version: self.minecraft_version,
                    assets_index: self.assets_index,
                    main_class: self.main_class,
                    component: self.component,
                    classpath: self.classpath,
                    supports_quick_play_single: self.supports_quick_play_multi,
                    supports_quick_play_multi: self.supports_quick_play_multi,
               },
               loader: Some(LoaderConfig {
                    loader: self.loader,
                    version: self.version
               }),
               directory: DirectoryConfig {
                    game_dir: self.game_dir,
                    lib_dir: self.lib_dir,
                    assets_dir: self.assets_dir,
               }
          }
     }

     pub fn save(self, path: &str) -> std::io::Result<Main> {
          let cfg = self.build();
          cfg.save(path)?;
          Ok(cfg)
     }
}