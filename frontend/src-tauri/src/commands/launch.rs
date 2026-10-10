use java::launch;

#[tauri::command]
pub async fn launch_command(inst_name: String, world: Option<&str>, server: Option<&str> ) -> Result<(), String> {     
     launch(&inst_name, world, server).await.map_err(|e| e.to_string())
}