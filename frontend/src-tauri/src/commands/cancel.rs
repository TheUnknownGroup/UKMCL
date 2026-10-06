use folders::hub_fold::make_hub;
use config::msa::AuthMSA;
#[tauri::command]
pub async fn cancel() -> Result<(), String> {
     let home = make_hub().map_err(|e| e.to_string())?;
     let conf = home.join("msa.toml").to_string_lossy().to_string();
     AuthMSA::delete(&conf).map_err(|e| e.to_string())?;

     Ok(())
}