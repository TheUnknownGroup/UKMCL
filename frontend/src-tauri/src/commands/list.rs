use list::list;

#[tauri::command]
pub fn get_command() -> Result<Vec<String>, String> {
   list().map_err(|e| e.to_string())
}