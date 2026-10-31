use anyhow::{Context, Result, bail};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use utils::toasts::{ToastKind, toast};

use crate::XSTS_URL;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct XstsAuth {
     pub properties: XstsProps,
     pub relying_party: String,
     pub token_type: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct XstsProps {
     pub sandbox_id: String,
     pub user_tokens: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct XstsToken {
     pub token: String,
}

pub async fn get_xsts(app: &AppHandle, client: &Client, token: &str) -> Result<String> {
     let body = XstsAuth {
          properties: XstsProps {
               sandbox_id: "RETAIL".to_string(),
               user_tokens: vec![token.to_string()],
          },
          relying_party: "rp://api.minecraftservices.com/".to_string(),
          token_type: "JWT".to_string(),
     };

     let resp = client.post(XSTS_URL).header("Accept", "application/json").json(&body).send().await.context("XSTS auth failed")?;
     if !resp.status().is_success() {
          let text = resp.text().await?;
          if text.contains("2148916233") {
               toast(app, ToastKind::Error, "This account doesn't own Minecraft.");
               bail!("This account doesn't own Minecraft");
          }

          if text.contains("2148916238") {
               toast(app, ToastKind::Error, "Xbox live is not avaiable in your country");
               bail!("Xbox live is not avaiable in your country");
          }
          toast(app, ToastKind::Error, format!("XSTS auth failed: {}", text));
          bail!("XSTS auth failed: {}", text);
     }

     let token: XstsToken = resp.json().await.context("failed to parse XSTS token")?;
     Ok(token.token)
}