use std::{error::Error, path::{Path, PathBuf}};

use utils::{folders::make_hub, offline};

use crate::{account::{AccountBuilder, AccountStruct}, instance::MainBuilder};

pub struct InstanceCfg<'a> {
     pub name: &'a str,
     pub mc_id: &'a str,
     pub asset_index: &'a str,
     pub main_class: &'a str,
     pub component: &'a str,
     pub jvm_args: &'a [String],
     pub game_args: &'a [String],
     pub supports_quick_play_single: bool,
     pub supports_quick_play_multi: bool,
     pub classpath: &'a [String],
     pub hub: &'a Path,
     pub loader: &'a str,
     pub loader_ver: &'a str, 
}

pub fn instance_config(c: &InstanceCfg) -> Result<(), Box<dyn Error>> {
     let text = |p: PathBuf| p.to_string_lossy().into_owned();
     let inst_root = c.hub.join("instances").join(c.name);
     let insts_dir = text(inst_root.join("minecraft"));
     let lib_dir = text(c.hub.join("libraries"));
     let assets_dir = text(c.hub.join("assets"));
     let versions_dir = text(c.hub.join("versions").join(c.mc_id).join(c.name).join("client.jar"));
     let cfg_path = text(inst_root.join("config.toml"));
     let mut cfg = MainBuilder::new(c.name, c.mc_id)
          .ver(c.asset_index, c.main_class, c.classpath, &versions_dir)
          .directory(&insts_dir, &lib_dir, &assets_dir)
          .java(c.component)
          .supports(c.supports_quick_play_multi, c.supports_quick_play_single)
          .launch_args(c.jvm_args, c.game_args);
     
     if c.loader != "vanilla" {
          cfg = cfg.loader(c.loader, c.loader_ver);
     }

     if c.hub.join("account.toml").exists() {
          let ram_info = AccountStruct::load(&c.hub.join("account.toml").to_string_lossy().as_ref()).map_err(|e| e.to_string())?;
          cfg = cfg.ram(ram_info.java.max, ram_info.java.min); 
     }

     cfg.build().save(&cfg_path)?;
     Ok(())
}

pub async fn offline_account(name: &str, min: u32, max: u32) -> Result<(), Box<dyn Error>> {
     let home = make_hub()?;
     if home.join("account.toml").exists() {
          return Ok(())
     }
     let cfg = AccountBuilder::new().account(name, &offline(name).hyphenated().to_string(), "0", "legacy").java(min, max).build();
     cfg.save(&home.join("account.toml").to_string_lossy().as_ref())?;
     Ok(())
}