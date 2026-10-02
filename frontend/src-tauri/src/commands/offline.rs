use auth::offline::offline;
use config::main_conf::{AccountConfig, Main};
use folders::hub_fold::make_hub;

#[tauri::command]
pub fn offline_account(name: &str) -> Result<(), String> {
     let uuid = offline(name).hyphenated().to_string();
     let main2 = make_hub().map_err(|e| e.to_string())?;
     let file = &main2.join("config.toml");

     let main_conf = AccountConfig::new(name, &uuid, "0", "legacy");
     let conf = &file.to_string_lossy().to_string();
     Main::update_acc(conf, main_conf).map_err(|e| e.to_string())?;

     Ok(())
}