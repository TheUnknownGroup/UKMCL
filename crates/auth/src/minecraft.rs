use anyhow::{Context, Result, bail};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use utils::toasts::{ToastKind, toast};

use crate::{MC_AUTH, MC_PROF};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftAuth {
     pub identity_token: String,
}

#[derive(Debug, Deserialize)]
pub struct MinecraftToken {
     pub access_token: String,
}

#[derive(Debug, Deserialize)]
pub struct MinecraftProfile {
     pub id: String,
     pub name: String,
}

pub async fn get_minecraft(app: &AppHandle, client: &Client, uhs: &str, token: &str) -> Result<MinecraftToken> {
     let body = MinecraftAuth {
          identity_token: format!("XBL3.0 x={};{}", uhs, token),
     };
     let resp = client.post(MC_AUTH).header("Accept", "application/json").json(&body).send().await.context("Minecraft auth req failed")?;
     if resp.status() == StatusCode::FORBIDDEN {
          let text = resp.text().await?;
          if text.contains("Invaild app registration") {
               toast(app, ToastKind::Error, "Minecraft services rejected this app. Submit it via aka.ms/AppRegInfo.");
               bail!("Minecraft services rejected this app. Submit it via aka.ms/AppRegInfo.");
          }
          toast(app, ToastKind::Error, format!("Minecraft auth forbidden: {}", text));
          bail!("Minecraft auth forbidden: {}", text);
     }

     if !resp.status().is_success() {
          let text = resp.text().await?;
          toast(app, ToastKind::Error, format!("Minecraft auth forbidden: {}", text));
          bail!("Minecraft auth failed: {}", text);
     }
     resp.json().await.context("failed to parse Minecraft token")
}

pub async fn get_profile(app: &AppHandle, client: &Client, token: &str) -> Result<MinecraftProfile> {
     let resp = client.get(MC_PROF).header("Authorization", format!("Bearer {}", token)).send().await.context("profile reqwest failed")?;
     if !resp.status().is_success() {
          let text = resp.text().await?;
          toast(app, ToastKind::Error, format!("profile fetch failed: {}", text));
          bail!("profile fetch failed: {}", text);
     }

     resp.json().await.context("failed to parse")
}