use delete::inst_del;

#[tauri::command]
pub fn delete_command(inst_name: String) -> Result<(), String> {
     inst_del(&inst_name).map_err(|e| e.to_string())
}