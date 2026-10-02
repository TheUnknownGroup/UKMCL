use config::main_conf::{JavaConfig, Main};
use folders::hub_fold::make_hub;

#[tauri::command]
pub fn ram(min: Option<u32>, max: Option<u32>) -> Result<(), String> {
     let main2 = make_hub().map_err(|e| e.to_string())?;
     let conf = &main2.join("config.toml").to_string_lossy().to_string();
     let exist = Main::load(&conf).unwrap_or_default();
     let main_conf = JavaConfig::new(min.unwrap_or(exist.java.min), max.unwrap_or(exist.java.max));
     Main::update_ram(conf, main_conf).map_err(|e| e.to_string())?;

     Ok(())
}