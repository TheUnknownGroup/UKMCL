use anyhow::{Context, Result, bail};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use utils::toasts::{ToastKind, toast};

use crate::XBOX_URL;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct XboxReq {
     pub properties: XboxProps,
     pub relying_party: String,
     pub token_type: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct XboxProps {
     pub auth_method: String,
     pub site_name: String,
     pub rps_ticket: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct XboxResp {
     pub token: String,
     pub display_claims: XboxClaims,
}

#[derive(Debug, Deserialize)]
pub struct XboxClaims {
     pub xui: Vec<Xui>,
}

#[derive(Debug, Deserialize)]
pub struct Xui {
     pub uhs: String,
}

pub async fn get_xbox(app: &AppHandle, client: &Client, token: &str) -> Result<(String, String)> {
     let body = XboxReq {
          properties: XboxProps {
               auth_method: "RPS".to_string(),
               site_name: "user.auth.xboxlive.com".to_string(),
               rps_ticket: format!("d={}", token),
          },
          relying_party: "http://auth.xboxlive.com".to_string(),
          token_type: "JWT".to_string(),
     };

     let resp = client.post(XBOX_URL)
          .header("Accept", "application/json")
          .json(&body)
          .send().await.context("Xbox auth req failed")?;

     if !resp.status().is_success() {
          let text = resp.text().await?;
          toast(app, ToastKind::Error, format!("Xbox auth failed: {}", text));
          bail!("Xbox auth failed: {}", text);
     }

     let token: XboxResp = resp.json().await.context("failed to parse token resp")?;
     let uhs = token.display_claims.xui.first().context("missing XUI claim in xbox resp")?.uhs.clone();

     Ok((token.token, uhs))
}