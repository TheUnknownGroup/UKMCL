use std::path::PathBuf;

use crate::MainBuilder;
use crate::main_conf::AccountBuilder;

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
               .java(4096, 2048)
               .loaders(load, version)
               .build();
     } else {
          cfg = MainBuilder::new(inst_name, id)
               .ver(ind, main, class.into(), versions_dir)
               .directory(insts_dir, lib_dir, assets_dir)
               .java(4096, 2048)
               .build();
     }

     let main_conf = AccountBuilder::new().account("User", "0", "legacy").build();
     
     main_conf.save(&main2.join("config.toml").to_string_lossy().to_string())?;
     
     cfg.save(&inst_dir)?;

     Ok(())
}

