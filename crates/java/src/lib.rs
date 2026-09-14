use std::{error::Error, process::{Command, Stdio}};
use config::Main;
use minecraft::client::offline::offline;
use folders::hub_fold::make_hub;

#[allow(unused)]
pub fn launch(inst_name: &str) -> Result<(), Box<dyn Error>>{
    let home_dir = make_hub()?;
    let inst_config = home_dir.join("instances").join(inst_name).join("config.toml").to_string_lossy().to_string();
    let config = Main::load(&inst_config).unwrap();

    let username = "TheGremlinX";
    let uuid = offline(username);

    let sep = if cfg!(windows) { ";" } else { ":" };
    let full = config.main.classpath.join(sep);

    let mut cmd = Command::new("java");
    cmd.current_dir(&config.main.directory.game_dir)
        .arg(format!("-Djava.library.path={}", config.main.directory.lib_dir))
        .arg("-Xmx2G")
        .arg("-cp").arg(full)
        .arg(config.main.main_class)
        .arg("--username").arg(username)
        .arg("--uuid").arg(uuid.hyphenated().to_string())
        .arg("--accessToken").arg("0")
        .arg("--userType").arg("legacy")
        .arg("--version").arg(config.main.minecraft_version)
        .arg("--gameDir").arg(config.main.directory.game_dir)
        .arg("--assetsDir").arg(config.main.directory.assets_dir)
        .arg("--assetIndex").arg(config.main.assets_index)
        .stdout(Stdio::piped());

    let mut child = cmd.spawn()?;
    
    Ok(())
}