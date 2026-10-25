use tauri::{AppHandle, WebviewWindowBuilder, WebviewUrl, Manager};

const WIDTH: f64 = 1250.0;
const HEIGHT: f64 = 700.0;

pub async fn spawn_mods(app_handle: &AppHandle, url: &str) -> Result<(), Box<dyn std::error::Error>> {
     if let Some(win) =  app_handle.get_webview_window("store") {
          win.show()?;
          win.unminimize()?;
          win.set_focus()?;
          return Ok(())
     }

     WebviewWindowBuilder::new(
          app_handle, "store", WebviewUrl::App(url.into()),
     )
    .title("UKMCL | Store")
    .inner_size(WIDTH, HEIGHT)
    .center()
    .resizable(false)
    .build()?;
     
     Ok(())
}