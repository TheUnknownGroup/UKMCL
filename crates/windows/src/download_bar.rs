use tauri::{AppHandle, WebviewWindowBuilder, WebviewUrl, Manager};

const WIDTH: f64 = 500.0;
const HEIGHT: f64 = 24.0;

pub async fn spawn(app_handle: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
     if app_handle.get_webview_window("download-bar").is_some() {
          return Ok(())
     }

     WebviewWindowBuilder::new(
          app_handle, "download-bar", WebviewUrl::App("download-bar".into()),
     )
    .title("Downloading assets...")
    .inner_size(WIDTH, HEIGHT)
    .center()
    .resizable(false)
    .build()?;
     
     Ok(())
}