use auth::microsoft::auth;
use config::main_conf::{AccountConfig, Main};
use folders::hub_fold::make_hub;
use tauri::AppHandle;

#[tauri::command]
pub async fn microsoft_auth(app: AppHandle) -> Result<(), String> {
     let info = auth(app.clone()).await.map_err(|e| e.to_string())?;

     let main2 = make_hub().map_err(|e| e.to_string())?;
     let file = &main2.join("config.toml");

     let main_conf = AccountConfig::new(&info.username, &info.uuid, &info.access_token, &info.user_type, &info.refresh_token, info.expires);
     let conf = &file.to_string_lossy().to_string();
     Main::update_acc(conf, main_conf).map_err(|e| e.to_string())?;

     Ok(())
}