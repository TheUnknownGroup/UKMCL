use tauri::{AppHandle, WebviewWindowBuilder, WebviewUrl, Manager};

const WIDTH: f64 = 854.0;
const HEIGHT: f64 = 480.0;

pub async fn spawn_mod(app_handle: &AppHandle, label: &String, url: &String, name: &String) -> Result<(), Box<dyn std::error::Error>> {
     if let Some(win) = app_handle.get_webview_window(label) {
          win.show()?;
          win.unminimize()?;
          win.set_focus()?;
          return Ok(())
     }

     WebviewWindowBuilder::new(
          app_handle, label, WebviewUrl::App(url.into()),
     )
    .title(format!("Downloading {}...", name))
    .inner_size(WIDTH, HEIGHT)
    .center()
    .resizable(false)
    .build()?;
     
     Ok(())
}