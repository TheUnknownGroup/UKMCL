use std::process::Stdio;
use tokio::process::Command;
use auth::offline::auth;

pub async fn launch_mc(
    java_path: &str,
    game_dir: &str,
    version: &str,
    natives_dir: &str,
    classpath: &str,
    main_class: &str,
    user: &str,
    server: Option<&str>,
    asset_index: &str,
) -> std::io::Result<()> {
    let uuid = auth(user);

    let mut cmd = Command::new("java");

    cmd.current_dir(game_dir)
        .arg("-Xmx2G")
        .arg("-cp")
        .arg(classpath)
        .arg(main_class)
        .arg("--username").arg(user)
        .arg("--uuid").arg(uuid.hyphenated().to_string())
        .arg("--accessToken").arg("0")
        .arg("--userType").arg("legacy")
        .arg("--version").arg(version)
        .arg("--gameDir").arg(game_dir)
        .arg("--assetsDir").arg(format!("{}/assets", game_dir))
        .arg("--assetIndex").arg(asset_index);

    if let Some(addr) = server {
        let (host, port) = addr.split_once(':').unwrap_or((addr, "25565"));
        cmd.arg("--server").arg(host).arg("--port").arg(port);
    }
    Ok(())
}