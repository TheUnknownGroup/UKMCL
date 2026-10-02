use java::launch;

#[tauri::command]
pub async fn launch_command(inst_name: String) -> Result<(), String> {
     launch(&inst_name).map_err(|e| e.to_string())
}