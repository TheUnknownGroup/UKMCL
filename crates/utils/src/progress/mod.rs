use serde::Serialize;
use tauri::{AppHandle, Emitter};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Progress {
     pub phase: String,
     pub downloaded: u64,
     pub total: u64,
     pub current: String,
     pub files: u32,
     pub file_total: u32,
}

pub fn emit_progress(app: &AppHandle, phase: &str, downloaded: u64, total: u64, current: impl Into<String>, files: u32, file_total: u32) -> tauri::Result<()> {
     app.emit("progress", Progress {
          phase: phase.to_owned(),
          downloaded,
          total,
          current: current.into(),
          files,
          file_total,
     })
}