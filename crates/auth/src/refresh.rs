use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use reqwest::Client;
use serde::Deserialize;
use tauri::AppHandle;
use utils::{client::base, toasts::{ToastKind, toast}};

use crate::{Authenticated, CLIENT_ID, TOKEN_URL, minecraft::{get_minecraft, get_profile}, xbox::get_xbox, xsts::get_xsts};

#[derive(Debug, Deserialize)]
pub struct RefreshToken {
     pub access_token: String,
     pub refresh_token: Option<String>,
     pub expires_in: u64,
}

async fn refresh_microsoft(app: &AppHandle, client: &Client, token: &str) -> Result<RefreshToken> {
     let params = [
          ("client_id", CLIENT_ID),
          ("refresh_token", token),
          ("grant_type", "refresh_token"),
          ("scope", "XboxLive.signin offline_access")
     ];
     let resp = client.post(TOKEN_URL).form(&params).send().await.context("refresh token request failed")?;
     if !resp.status().is_success() {
          let text = resp.text().await?;
          if text.contains("invalid_grant") {
               toast(app, ToastKind::Error, "Refresh token failed expired");
               bail!("Refresh token failed expired");
          }
          toast(app, ToastKind::Error, format!("refresh token failed: {}", text));
          bail!("refresh token failed: {}", text);
     }
     resp.json().await.context("failed to parse refresh req")
}

pub async fn refresh(app: &AppHandle, token: &str) -> Result<Authenticated> {
     let client = base();
     let ms = refresh_microsoft(app, &client, token).await?;
     let (xbox, uhs) = get_xbox(app, &client, &ms.access_token).await?;
     let xsts = get_xsts(app, &client, &xbox).await?;
     let mc = get_minecraft(app, &client, &uhs, &xsts).await?;
     let profile = get_profile(app, &client, &mc.access_token).await?;
     let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
     
     Ok(Authenticated { 
          username: profile.name,
          uuid: profile.id,
          access_token: mc.access_token,
          user_type: "msa".into(),
          refresh_token: ms.refresh_token.or_else(|| Some(token.to_string())),
          expires: now + ms.expires_in,
     })
}