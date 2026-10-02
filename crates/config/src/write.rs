use std::path::PathBuf;

use crate::{MainBuilder};

pub fn cfg_write(inst_name: &str, id: &str, ind: &str, main: &str, main2: PathBuf, class: &[String], load: &str, version: &str) -> Result<(), Box<dyn std::error::Error>>{
     let insts_dir = &main2.join("instances").join(inst_name).join("minecraft").to_string_lossy().to_string();
     let lib_dir = &main2.join("libraries").to_string_lossy().to_string();
     let assets_dir = &main2.join("assets").to_string_lossy().to_string();

     let versions_dir = &main2.join("versions").join(id).join(inst_name).join("client.jar").to_string_lossy().to_string();

     let inst_dir = &main2.join("instances").join(inst_name).join("config.toml").to_string_lossy().to_string();

     let cfg;

     if load != "vanilla" {
          cfg = MainBuilder::new(inst_name, id)
               .ver(ind, main, class.into(), versions_dir)
               .directory(insts_dir, lib_dir, assets_dir)
               .loaders(load, version)
               .build();
     } else {
          cfg = MainBuilder::new(inst_name, id)
               .ver(ind, main, class.into(), versions_dir)
               .directory(insts_dir, lib_dir, assets_dir)
               .build();
     }
     
     cfg.save(&inst_dir)?;

     Ok(())
}

