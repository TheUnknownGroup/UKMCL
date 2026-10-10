use delete::inst_del;
use folders::hub_fold::make_hub;

#[tauri::command]
pub fn delete_command(inst_name: String) -> Result<(), String> {
     inst_del(&inst_name).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_mod_json(id: &str, ver: &str) -> Result<(), String> {
     let home = make_hub().map_err(|e| e.to_string())?;
     let jsons = home.join("jsons").join(format!("{}-{}.json", id, ver));
     std::fs::remove_file(jsons).map_err(|e| e.to_string())
}