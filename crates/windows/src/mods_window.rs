use tauri::{AppHandle, WebviewWindowBuilder, WebviewUrl, Manager};

const WIDTH: f64 = 1080.0;
const HEIGHT: f64 = 600.0;

pub async fn spawn_mods(app_handle: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
     if app_handle.get_webview_window("mods").is_some() {
          return Ok(())
     }

     WebviewWindowBuilder::new(
          app_handle, "mods", WebviewUrl::App("mods".into()),
     )
    .title("UKMCL | Mods")
    .inner_size(WIDTH, HEIGHT)
    .center()
    .resizable(false)
    .build()?;
     
     Ok(())
}