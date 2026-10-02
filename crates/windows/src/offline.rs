use tauri::{AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder};

const WIDTH: f64 = 450.0;
const HEIGHT: f64 = 325.0;

pub async fn spawn_offs(app_handle: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
     if app_handle.get_webview_window("offline").is_some() {
          return Ok(())
     }
     let window = WebviewWindowBuilder::new(
          app_handle, "offline", WebviewUrl::App("offline".into()),
     )
    .title("Offline Account Creation")
    .inner_size(WIDTH, HEIGHT)
    .center()
    .resizable(false)
    .build()?;

     let app = app_handle.clone();
     window.on_window_event(move |e| {
          if let tauri::WindowEvent::Destroyed = e {
               let _ = app.emit("closing", ());
          }
     });
     
     Ok(())
}