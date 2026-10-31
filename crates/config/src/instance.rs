use std::{io::Result, path::Path};

use serde::{Deserialize, Serialize};

use crate::atomic::atomic_write;
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
     ram_min: u32,
     ram_max: u32,
     jvm_args: Vec<String>,
     game_args: Vec<String>,
     supports_quick_play_single: bool,
     supports_quick_play_multi: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstanceStruct {
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
     pub ram_min: u32,
     pub ram_max: u32,
     #[serde(default)]
     pub jvm_args: Vec<String>,
     #[serde(default)]
     pub game_args: Vec<String>,
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

impl InstanceStruct {
     pub fn save(&self, path: &str) -> Result<()> {
          let toml_str = toml::to_string_pretty(self).unwrap();
          atomic_write(Path::new(path), toml_str.as_bytes())
     }

     pub fn load(path: &str) -> Result<Self> {
          let data = std::fs::read_to_string(path)?;
          let config: Self = toml::from_str(&data).expect("Failed to parse");
          Ok(config)
     }
}

impl MainBuilder {
     pub fn new(name: &str, id: &str) -> Self {
          Self {
               instance: name.into(),
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
               ram_min: 2048,
               ram_max: 4096,
               jvm_args: Vec::new(),
               game_args: Vec::new(),
               supports_quick_play_multi: false,
               supports_quick_play_single: false,
          }
     }

     pub fn ver(mut self, index: &str, class: &str, classpath: &[String], client_jar: &str) -> Self {
          self.assets_index = index.into();
          self.main_class = class.into();
          self.classpath = MainBuilder::build_with_jar(classpath, client_jar);
          self
     }

     fn build_with_jar(classes: &[String], client_jar: &str) -> Vec<String> {
          let mut all = classes.to_vec();
          all.push(client_jar.into());
          all
     }

     pub fn ram(mut self, max: u32, min: u32) -> Self {
          self.ram_max = max;
          self.ram_min = min;
          self
     }

     pub fn directory(mut self, directory: &str, lib_dir: &str, assets_dir: &str) -> Self {
          self.game_dir = directory.into();
          self.lib_dir = lib_dir.into();
          self.assets_dir = assets_dir.into();
          self
     }

     pub fn loader(mut self, loader: &str, version: &str) -> Self {
          self.loader = loader.into();
          self.version = version.into();
          self
     }

     pub fn java(mut self, component: &str) -> Self {
          self.component = component.into();
          self
     }

     pub fn launch_args(mut self, jvm: &[String], game: &[String]) -> Self {
          self.jvm_args = jvm.to_vec();
          self.game_args = game.to_vec();
          self
     }

     pub fn supports(mut self, multi: bool, single: bool) -> Self {
          self.supports_quick_play_multi = multi;
          self.supports_quick_play_single = single;
          self
     }

     pub fn build(self) -> InstanceStruct {
          InstanceStruct { 
               main: InstanceConfig {
                    instance: self.instance,
                    minecraft_version: self.minecraft_version,
                    assets_index: self.assets_index,
                    main_class: self.main_class,
                    component: self.component,
                    classpath: self.classpath,
                    ram_max: self.ram_max,
                    ram_min: self.ram_min,
                    jvm_args: self.jvm_args,
                    game_args: self.game_args,
                    supports_quick_play_multi: self.supports_quick_play_multi,
                    supports_quick_play_single: self.supports_quick_play_single,
               }, 
               loader: Some(LoaderConfig {
                         loader: self.loader,
                         version: self.version,
               }), 
               directory: DirectoryConfig {
                    game_dir: self.game_dir,
                    lib_dir: self.lib_dir,
                    assets_dir: self.assets_dir,
               } 
          }
     }

     pub fn save(self, path: &str) -> Result<InstanceStruct> {
          let cfg = self.build();
          cfg.save(path)?;
          Ok(cfg)
     }
}