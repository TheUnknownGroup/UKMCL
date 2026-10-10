pub mod manager;

use std::{error::Error, process::{Command, Stdio}};
use config::Main;
use config::main_conf;
use folders::hub_fold::make_hub;

use crate::manager::ensure;

#[allow(unused)]
pub async fn launch(inst_name: &str, world: Option<&str>, server: Option<&str>) -> Result<(), Box<dyn Error>>{
     let home_dir = make_hub()?;
     let inst_config = home_dir.join("instances").join(inst_name).join("config.toml").to_string_lossy().to_string();
     let main_file = home_dir.join("config.toml").to_string_lossy().to_string();
     let inst_file = Main::load(&inst_config).unwrap();
     let inst_main = &inst_file.main;
     let inst_dir = &inst_file.directory;
     let main = main_conf::Main::load(&main_file).unwrap();
     let account = main.account;
     let inst_java = main.java;

     let comp = &inst_main.component;
     let java = ensure(comp).await?;

     let sep = if cfg!(windows) { ";" } else { ":" };
     let full = &inst_main.classpath.join(sep);

     let mut cmd = Command::new(&java);
     cmd.current_dir(&inst_dir.game_dir)
          .arg(format!("-Djava.library.path={}",&inst_dir.lib_dir))
          .arg(format!("-Xms{}M", &inst_java.min)).arg(format!("-Xmx{}M", &inst_java.max))
          .arg("-cp").arg(full)
          .arg(&inst_main.main_class)
          .arg("--username").arg(&account.username)
          .arg("--uuid").arg(&account.uuid)
          .arg("--accessToken").arg(&account.access_token)
          .arg("--userType").arg(&account.user_type)
          .arg("--version").arg(&inst_main.minecraft_version)
          .arg("--gameDir").arg(&inst_dir.game_dir)
          .arg("--assetsDir").arg(&inst_dir.assets_dir)
          .arg("--assetIndex").arg(&inst_main.assets_index);

     let quick_play = match (&world, &server) {
          (Some(w), _) if inst_main.supports_quick_play_single => Some(("--quickPlaySingleplayer", w)),
          (_, Some(s)) if inst_main.supports_quick_play_multi => Some(("--quickPlayMultiplayer", s)),
          _ => None,
     };

     if let Some((flag, val)) = quick_play {
          cmd.arg(flag).arg(val);
     }

     if comp == "java-runtime-epsilon" {
          cmd.arg("--enable-native-access=ALL-UNNAMED").arg("--sun-misc-unsafe-memory-access=allow").arg("java.base/jdk.internal.misc=ALL-UNNAMED");
     }

     cmd.stdout(Stdio::piped());

     let mut child = cmd.spawn()?;
    
     Ok(())
}