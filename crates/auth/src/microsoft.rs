use std::{sync::Arc, time::Duration};

use anyhow::{bail, Result, Context};
use reqwest::Client;
use serde::Deserialize;
use tauri::{AppHandle, Listener};
use tokio::{sync::Notify, time::sleep};
use utils::toasts::{ToastKind, toast};

use crate::{CLIENT_ID, TOKEN_URL, device_code::DeviceResp};

#[derive(Debug, Deserialize)]
pub struct MicrosoftToken {
     pub access_token: String,
     pub refresh_token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OAuthErr {
     pub error: String,
}

pub async fn poll_for_token(app: &AppHandle, client: &Client, device_code: &DeviceResp) -> Result<MicrosoftToken> {
     let params = [
          ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
          ("client_id", CLIENT_ID),
          ("device_code", &device_code.device_code),
     ];

     let mut inte = Duration::from_secs(device_code.interval);
     let cancel = Arc::new(Notify::new());
     let listener = {
          let cancel = cancel.clone();
          app.listen("cancels", move |_| cancel.notify_one())
     };

     toast(app, ToastKind::Info, "starting sign in loop");

     let result = async {
          loop {
               tokio::select! {
                    _ = cancel.notified() => bail!("sign-in canceled"),
                    _ = sleep(inte) => {}
               }
               let resp = client.post(TOKEN_URL).form(&params).send().await.context("token request failed")?;
     
               if resp.status().is_success() {
                    return resp.json().await.context("failed to parse token resp");
               }
     
               let error: OAuthErr = resp.json().await.context("failed to parse OAuth Error")?;
               
               match error.error.as_str() {
                    "authorization_pending" => continue,
                    "slow_down" => {
                         inte += Duration::from_secs(5);
                         continue;
                    }
                    "authorization_declined" => bail!("user declined auth"),
                    "expired_token" => bail!("device code expired"),
                    other => bail!("OAuth error: {}", other),
               }
          }
     }
     .await;
     app.unlisten(listener);
     result
}