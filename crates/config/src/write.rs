use std::path::PathBuf;

use crate::{MainBuilder};

pub fn cfg_write(inst_name: &str, id: &str, ind: &str, main: &str, main2: PathBuf, class: &[String], load: &str, version: &str, component: &str, single: bool, multi: bool, jvm: &[String], game: &[String]) -> Result<(), Box<dyn std::error::Error>>{
     let insts_dir = &main2.join("instances").join(inst_name).join("minecraft").to_string_lossy().to_string();
     let lib_dir = &main2.join("libraries").to_string_lossy().to_string();
     let assets_dir = &main2.join("assets").to_string_lossy().to_string();

     let versions_dir = &main2.join("versions").join(id).join(inst_name).join("client.jar").to_string_lossy().to_string();

     let inst_dir = &main2.join("instances").join(inst_name).join("config.toml").to_string_lossy().to_string();

     let mut cfg = MainBuilder::new(inst_name, id)
          .ver(ind, main, class, versions_dir)
          .directory(insts_dir, lib_dir, assets_dir)
          .javas(component).supports(multi, single)
          .launch_args(jvm, game);

     if load != "vanilla" {
          cfg = cfg.loaders(load, version);
     }
     
     cfg.build().save(inst_dir)?;

     Ok(())
}

