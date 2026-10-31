use tauri::{AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder};
use urlencoding::encode;

const STORE_WIDTH: f64 = 1250.0;
const STORE_HEIGHT: f64 = 700.0;

pub async fn spawn_store(app_handle: &AppHandle, url: &str) -> Result<(), Box<dyn std::error::Error>> {
     if let Some(win) = app_handle.get_webview_window("store") {
          win.show()?;
          win.unminimize()?;
          win.set_focus()?;
          return Ok(())
     }
     WebviewWindowBuilder::new(app_handle, "store", WebviewUrl::App(url.into()))
          .title("Store Page")
          .inner_size(STORE_WIDTH, STORE_HEIGHT)
          .center().resizable(false).decorations(false).transparent(true).shadow(true)
          .build()?;

     Ok(())
}

const CREATION_WIDTH: f64 = 830.0;
const CREATION_HEIGHT: f64 = 450.0;

pub async fn spawn_instance_creation(app_handle: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
     if let Some(win) = app_handle.get_webview_window("instance_creation") {
          win.show()?;
          win.unminimize()?;
          win.set_focus()?;
          return Ok(())
     }

     let window = WebviewWindowBuilder::new(
          app_handle, "instance_creation", WebviewUrl::App(format!("/instance_creation?title={}", encode("Creating an Instance")).into()),
     ).title("Creating an instance")
     .inner_size(CREATION_WIDTH, CREATION_HEIGHT).center().transparent(true).shadow(true)
     .resizable(false).decorations(false).build()?;
     let app = app_handle.clone();
     window.on_window_event(move |e| {
          if let tauri::WindowEvent::Destroyed = e {
               let _ = app.emit("closing", ());
          }
     });
     
     Ok(())
}

const WIDTH: f64 = 1320.0;
const HEIGHT: f64 = 600.0;

pub async fn spawn_instance(app_handle: &AppHandle, label: &str, url: &str, name: &str) -> Result<(), Box<dyn std::error::Error>> {
     if let Some(win) = app_handle.get_webview_window(label) {
          win.show()?;
          win.unminimize()?;
          win.set_focus()?;
          return Ok(())
     }

     WebviewWindowBuilder::new(
          app_handle, label, WebviewUrl::App(url.into()),
     ).title(format!("Instance: {}", name))
     .inner_size(WIDTH, HEIGHT).center().transparent(true).shadow(true)
     .resizable(false).decorations(false).build()?;
     
     Ok(())
}