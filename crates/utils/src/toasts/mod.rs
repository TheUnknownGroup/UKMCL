use tauri::{AppHandle, Emitter};
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ToastKind {
     Success,
     Error,
     Info,
}

#[derive(Clone, Serialize)]
struct ToastPayload {
     kind: ToastKind,
     message: String,
}

pub fn toast(app: &AppHandle, kind: ToastKind, message: impl Into<String>) {
     let _ = app.emit("toast", ToastPayload { kind, message: message.into() });
}