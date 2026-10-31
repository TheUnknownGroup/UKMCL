use utils::{folders::make_hub};
use config::account::AccountStruct;

#[tauri::command]
pub async fn get_java_minimum() -> Result<u32, String> {
     let home = make_hub().map_err(|e| e.to_string())?;
     let config = home.join("account.toml");
     let java = AccountStruct::load(&config.to_string_lossy().as_ref()).map_err(|e| e.to_string())?;
     Ok(java.java.min)
}

#[tauri::command]
pub async fn get_java_maximum() -> Result<u32, String> {
     let home = make_hub().map_err(|e| e.to_string())?;
     let config = home.join("account.toml");
     let java = AccountStruct::load(&config.to_string_lossy().as_ref()).map_err(|e| e.to_string())?;
     Ok(java.java.max)
}