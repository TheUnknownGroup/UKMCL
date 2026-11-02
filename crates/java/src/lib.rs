use std::process::Stdio;
use anyhow::{Context, Result, bail};
use config::{account::AccountStruct, instance::InstanceStruct};
use tokio::process::Command;
use utils::folders::make_hub;
use crate::manager::ensure;

pub mod manager;

pub async fn launch(name: &str, world: Option<&str>, server: Option<&str>) -> Result<()> {
     let home = make_hub()?;
     let instance = home.join("instances").join(name).join("config.toml").to_string_lossy().to_string();
     let account = home.join("account.toml").to_string_lossy().to_string();
     let file = InstanceStruct::load(&instance).unwrap();
     let instance_main = &file.main;
     let instance_directory = &file.directory;
     let account_file = AccountStruct::load(&account).unwrap();
     let account_info = account_file.account;
     let instance_java = &file.main.java;

     let comp = &instance_main.component;
     let java = ensure(comp).await.context("failed to install or locate java runtime")?;
     let sep = if cfg!(windows) { ";" } else { ":" };
     let full = &instance_main.classpath.join(sep);
     let mut cmd = Command::new(&java);
     cmd.current_dir(&instance_directory.game_dir)
          .arg(format!("-Djava.library.path={}", instance_directory.lib_dir))
          .arg(format!("-Xms{}M", instance_java.min)).arg(format!("-Xmx{}M", instance_java.max));

     if comp == "java-runtime-epsilon" {
          cmd.arg("--enable-native-access=ALL-UNNAMED").arg("--sun-misc-unsafe-memory-access=allow").arg("--add-opens").arg("java.base/jdk.internal.misc=ALL-UNNAMED");
     }

     for a in &instance_main.jvm_args { cmd.arg(a); }

     cmd.arg("-cp").arg(full)
          .arg(&instance_main.main_class)
          .arg("--username").arg(&account_info.username)
          .arg("--uuid").arg(&account_info.uuid)
          .arg("--accessToken").arg(&account_info.access_token)
          .arg("--userType").arg(&account_info.user_type)
          .arg("--version").arg(&instance_main.minecraft_version)
          .arg("--gameDir").arg(&instance_directory.game_dir)
          .arg("--assetsDir").arg(&instance_directory.assets_dir)
          .arg("--assetIndex").arg(&instance_main.assets_index);

     let quick_play = match (&world, &server) {
          (Some(w), _) if instance_main.supports_quick_play_single => Some(("--quickPlaySingleplayer", w)),
          (_, Some(s)) if instance_main.supports_quick_play_multi => Some(("--quickPlayMultiplayer", s)),
          _ => None,
     };

     if let Some((f, v)) = quick_play {
          cmd.arg(f).arg(v);
     }

     for a in &instance_main.game_args { cmd.arg(a); }
     cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
     let mut child = cmd.spawn().with_context(|| format!("failed to start java: {}", java.display()))?;

     let status = child.wait().await.context("failed while waiting for minecraft")?;

     if !status.success() {
          bail!("minecraft exited with status: {}", status);
     }

     Ok(())
}