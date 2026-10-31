use reqwest::Client;
use serde::Deserialize;
use anyhow::{bail, Result, Context};
use utils::toasts::{ToastKind, toast};
use tauri::AppHandle;

use crate::{CLIENT_ID, DEVICE_URL};

#[derive(Debug, Deserialize)]
pub struct DeviceResp {
     pub device_code: String,
     pub user_code: String,
     pub verification_uri: String,
     pub expires_in: u64,
     pub interval: u64
}

pub async fn request_code(app: &AppHandle, client: &Client) -> Result<DeviceResp> {
     let params = [
          ("client_id", CLIENT_ID),
          ("scope", "XboxLive.signin offline_access")
     ];
     let resp = client.post(DEVICE_URL).form(&params).send().await.context("failed to get device code")?;
     if !resp.status().is_success() {
          let text = resp.text().await?;
          toast(app, ToastKind::Error, &text);
          bail!("device code request failed: {}", text);
     }

     resp.json().await.context("failed to parse")
}