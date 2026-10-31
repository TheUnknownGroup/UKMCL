use auth::auth;
use tauri::{AppHandle};
use utils::{folders::make_hub, toasts::{ToastKind, toast}};
use config::{account::{AccountConfig, AccountStruct}, auth_microsoft::AuthMSA, write::offline_account};

#[tauri::command]
pub async fn microsoft_auth(app: AppHandle) -> Result<(), String> {
     let info = auth(app.clone()).await.map_err(|e| e.to_string())?;
     let main = make_hub().map_err(|e| e.to_string())?;
     let file = &main.join("account.toml");
     let main_config = AccountConfig::new(&info.username, &info.uuid, &info.access_token, &info.user_type, &info.refresh_token, info.expires);
     let config = &file.to_string_lossy().to_string();
     AccountStruct::update_acc(config, main_config).map_err(|e| e.to_string())?;
     toast(&app, ToastKind::Success, "Microsoft Account successfully added.");
     Ok(())
}

#[tauri::command]
pub async fn get_auth_info() -> Result<AuthMSA, String> {
     let home = make_hub().map_err(|e| e.to_string())?;
     let cfg = home.join("msa.toml").to_string_lossy().to_string();
     let info = AuthMSA::load(&cfg).map_err(|e| e.to_string())?;
     Ok(info)
}

#[tauri::command]
pub async fn cancel() -> Result<(), String> {
     let home = make_hub().map_err(|e| e.to_string())?;
     let conf = home.join("msa.toml").to_string_lossy().to_string();
     AuthMSA::delete(&conf).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn account_info() -> Result<AccountConfig, String> {
     let home = make_hub().map_err(|e| e.to_string())?;
     let cfg = home.join("account.toml");
     let account = AccountStruct::load(&cfg.to_string_lossy().as_ref()).map_err(|e| e.to_string())?;

     Ok(account.account)
}

#[tauri::command]
pub async fn write_initial() -> Result<(), String> {
     offline_account("User", 1024, 4096).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn offline(name: &str) -> Result<(), String> {
     let uuid = utils::offline(name).hyphenated().to_string();
     let home = make_hub().map_err(|e| e.to_string())?;
     let file = home.join("account.toml").to_string_lossy().to_string();
     let acc = AccountConfig::new(name, &uuid, "0", "legacy", &None, 0);
     AccountStruct::update_acc(&file, acc).map_err(|e| e.to_string())?;
     Ok(())
}

#[tauri::command]
pub async fn sign_out() -> Result<(), String> {
     let home = make_hub().map_err(|e| e.to_string())?.join("account.toml").to_string_lossy().to_string();
     let acc = AccountConfig::new("", "", "", "", &None, 0);
     AccountStruct::update_acc(&home, acc).map_err(|e| e.to_string())?;
     Ok(())
}