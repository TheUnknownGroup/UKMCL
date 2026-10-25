use tauri::{AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder};

const WIDTH: f64 = 830.0;
const HEIGHT: f64 = 450.0;

pub async fn spawn_inst(app_handle: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
     if let Some(win) = app_handle.get_webview_window("inst_creation") {
          win.show()?;
          win.unminimize()?;
          win.set_focus()?;
          return Ok(())
     }

     let window = WebviewWindowBuilder::new(
          app_handle, "inst_creation", WebviewUrl::App("inst_creation".into()),
     )
    .title("Creating an instance.")
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