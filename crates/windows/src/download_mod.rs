use tauri::{AppHandle, WebviewWindowBuilder, WebviewUrl, Manager};

const WIDTH: f64 = 854.0;
const HEIGHT: f64 = 480.0;

pub async fn spawn_mod(app_handle: &AppHandle, label: &String, url: &String, name: &String) -> Result<(), Box<dyn std::error::Error>> {
     if app_handle.get_webview_window(label).is_some() {
          return Ok(())
     }

     WebviewWindowBuilder::new(
          app_handle, label, WebviewUrl::App(url.into()),
     )
    .title(format!("Downloading mod {}...", name))
    .inner_size(WIDTH, HEIGHT)
    .center()
    .resizable(false)
    .build()?;
     
     Ok(())
}