use std::time::{Duration, SystemTime, UNIX_EPOCH};
use reqwest::Client;
use tauri::{AppHandle, Emitter, Listener};
use tokio::time::sleep;
use folders::hub_fold::make_hub;
use anyhow::{bail, Result, Context};
use serde::{Deserialize, Serialize};

const CLIENT_ID: &str = match option_env!("MC_CLIENT_ID") { Some(id) => id, None => "fallback-id" };
const DEVICE_URL: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode";
const TOKEN_URL: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/token";
const XBOX_URL: &str = "https://user.auth.xboxlive.com/user/authenticate";
const XSTS_URL: &str = "https://xsts.auth.xboxlive.com/xsts/authorize";
const MC_AUTH_URL: &str = "https://api.minecraftservices.com/authentication/login_with_xbox";
const MC_PROFILE_URL: &str = "https://api.minecraftservices.com/minecraft/profile";

#[derive(Debug, Deserialize)]
struct DeviceResp {
     device_code: String,
     user_code: String,
     verification_uri: String,
     expires_in: u64,
     interval: u64,
}

#[derive(Debug, Deserialize)]
struct MicrosoftToken {
     access_token: String,
     refresh_token: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OAuthErr {
     error: String,
}

async fn request_code(client: &reqwest::Client) -> Result<DeviceResp> {
     let params = [
          ("client_id", CLIENT_ID),
          ("scope", "XboxLive.signin offline_access")
     ];

     let resp = client.post(DEVICE_URL).form(&params).send().await.context("failed to request device code")?;

     if !resp.status().is_success() {
          let text = resp.text().await?;
          bail!("device code request failed: {}", text);
     }
     
     resp.json().await.context("failed to parse device code resp")
}

async fn poll_for_token(client: &reqwest::Client, device_code: &DeviceResp) -> Result<MicrosoftToken> {
     let params = [
          ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
          ("client_id", CLIENT_ID),
          ("device_code", &device_code.device_code),
     ];

     let inte = Duration::from_secs(device_code.interval);

     loop {
          sleep(inte).await;
          let resp = client.post(TOKEN_URL).form(&params).send().await.context("token request failed")?;

          if resp.status().is_success() {
               return resp.json().await.context("failed to parse token resp");
          }

          let error: OAuthErr = resp.json().await.context("failed to parse OAuth Error")?;
          match error.error.as_str() {
               "authorization_pending" => continue,
               "authorization_declined" => bail!("user declined auth"),
               "expired_token" => bail!("device code expired"),
               other => bail!("OAuth error: {}", other),
          }
     }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct XboxAuthReq {
     properties: XboxAuthProps,
     relying_party: String,
     token_type: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct XboxAuthProps {
     auth_method: String,
     site_name: String,
     rps_ticket: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct XboxTokenResp {
     token: String,
     display_claims: XboxDisplayClaims,
}

#[derive(Debug, Deserialize)]
struct XboxDisplayClaims {
     xui: Vec<XuiClaim>,
}

#[derive(Debug, Deserialize)]
struct XuiClaim {
     uhs: String,
}

async fn get_xbox(client: &reqwest::Client, microsoft_token: &str) -> Result<(String, String)> {
     let body = XboxAuthReq {
          properties: XboxAuthProps {
               auth_method: "RPS".to_string(),
               site_name: "user.auth.xboxlive.com".to_string(),
               rps_ticket: format!("d={}", microsoft_token),
          },
          relying_party: "http://auth.xboxlive.com".to_string(),
          token_type: "JWT".to_string(),
     };

     let resp = client.post(XBOX_URL)
          .header("Accept", "application/json")
          .json(&body)
          .send()
          .await
          .context("Xbox auth req failed")?;
     
     if !resp.status().is_success() {
          let text = resp.text().await?;
          bail!("Xbox auth failed: {}", text);
     }

     let token: XboxTokenResp = resp.json().await.context("failed to parse token resp")?;

     let uhs = token.display_claims.xui.first().context("missing XUI claim in Xbox resp")?.uhs.clone();
     
     Ok((token.token, uhs))
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct XstsAuth {
     properties: XstsProps,
     relying_party: String,
     token_type: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct XstsProps {
     sandbox_id: String,
     user_tokens: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct XstsToken {
     token: String,
}

async fn get_xsts(client: &reqwest::Client, xbox_token: &str) -> Result<String> {
     let body = XstsAuth {
          properties: XstsProps { 
               sandbox_id: "RETAIL".to_string(),
               user_tokens: vec![xbox_token.to_string()] 
          },
          relying_party: "rp://api.minecraftservices.com/".to_string(),
          token_type: "JWT".to_string(),
     };

     let resp = client.post(XSTS_URL).header("Accept", "application/json").json(&body).send().await.context("XSTS auth failed")?;

     if !resp.status().is_success() {
          let text = resp.text().await?;
          if text.contains("2148916233") {
               bail!("This acccount doesn't own Minecraft");
          }

          if text.contains("2148916238") {
               bail!("Xbox live is not available in your country");
          }

          bail!("XSTS auth failed: {}", text);
     }

     let token: XstsToken = resp.json().await.context("failed to parse XSTS token")?;

     Ok(token.token)
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct MinecraftAuth {
     identity_token: String,
}

#[derive(Debug, Deserialize)]
struct MinecraftToken {
     access_token: String,
}

#[derive(Debug, Deserialize)]
struct MinecraftProfile {
     id: String,
     name: String,
}

async fn get_minecraft(client: &reqwest::Client, uhs: &str, xsts_token: &str) -> Result<MinecraftToken> {
     let body = MinecraftAuth {
          identity_token: format!("XBL3.0 x={};{}", uhs, xsts_token),
     };

     let resp = client.post(MC_AUTH_URL).header("Accept", "application/json").json(&body).send().await.context("Minecraft auth req failed")?;
     println!("working? {}", resp.status());
     if resp.status() == reqwest::StatusCode::FORBIDDEN {
          let text = resp.text().await?;
          println!("Minecraft auth forbidden: {}", text);
          if text.contains("Invalid app registration") {
               bail!("Minecraft Services rejected this app. Submit it via aka.ms/AppRegInfo.");
          }
          
          bail!("Minecraft auth forbidden: {}", text);
     }

     if !resp.status().is_success() {
          let text = resp.text().await?;
          bail!("Minecraft auth failed: {}", text);
     }

     resp.json().await.context("failed to parse Minecraft token")
}

async fn get_minecraft_prof(client: &reqwest::Client, access_token: &str) -> Result<MinecraftProfile> {
     let resp = client.get(MC_PROFILE_URL).header("Authorization", format!("Bearer {}", access_token)).send().await.context("profile reqwest failed")?;

     if !resp.status().is_success() {
          let text = resp.text().await?;
          bail!("profile fetch failed: {}", text);
     }

     resp.json().await.context("failed to parse")
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Authenticated {
     pub username: String,
     pub uuid: String,
     pub access_token: String,
     pub user_type: String,
     pub refresh_token: Option<String>,
     pub expires: u64,
}

use config::msa::AuthMSA;

pub async fn auth(app: AppHandle) -> Result<Authenticated> {
     let client = reqwest::Client::new();
     let home = make_hub()?;
     let main = home.join("msa.toml").to_string_lossy().to_string();
     let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
     let device = request_code(&client).await?;
     AuthMSA::new(&device.verification_uri, &device.user_code, now + device.expires_in).save(&main)?;
     app.emit("open_msa", ())?;
     println!("working?");     
     let mico_token = match poll_for_token(&client, &device).await {
          Ok(t) => t,
          Err(e) => {
               app.emit("close_msa", ())?;
               return Err(e);
          }
     };
          
     app.emit("close_msa", ())?;
     let result = async {
          let (xbox, uhs) = get_xbox(&client, &mico_token.access_token).await?;
          let xsts = get_xsts(&client, &xbox).await?;
          let mc = get_minecraft(&client, &uhs, &xsts).await?; 
          let profile = get_minecraft_prof(&client, &mc.access_token).await?;
          
          Ok::<_, anyhow::Error>(Authenticated {
               username: profile.name,
               uuid: profile.id,
               access_token: mc.access_token,
               user_type: "msa".into(),
               refresh_token: mico_token.refresh_token,
               expires: now + 86400,
          })
     }.await;

     let _ = AuthMSA::delete(&main)?;
      
     result
}

#[derive(Debug, Deserialize)]
struct RefreshToken {
     access_token: String,
     refresh_token: Option<String>,
     expires_in: u64,
}

async fn refresh_ms(client: &Client, refresh_token: &str) -> Result<RefreshToken> {
     let params = [
          ("client_id", CLIENT_ID),
          ("refresh_token", refresh_token),
          ("grant_type", "refresh_token"),
          ("scope", "XboxLive.signin offline_access")
     ];

     let resp = client.post(TOKEN_URL).form(&params).send().await.context("refresh token req failed")?;

     if resp.status().is_success() {
          let text = resp.text().await?;

          if text.contains("invalid_grant") {
               bail!("Refresh Token expired or revoked");
          }

          bail!("refresh token failed {}", text);
     }

     resp.json().await.context("failed to parse refresh req")
}

pub async fn refresh(refresh_token: &str) -> Result<Authenticated> {
     let client = Client::new();

     let ms = refresh_ms(&client, refresh_token).await?;
     let (xbox, uhs) = get_xbox(&client, &ms.access_token).await?;
     let xsts = get_xsts(&client, &xbox).await?;
     let mc = get_minecraft(&client, &uhs, &xsts).await?;
     let profile = get_minecraft_prof(&client, &mc.access_token).await?;

     let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

     Ok(Authenticated { 
          username: profile.name, 
          uuid: profile.id, 
          access_token: mc.access_token, 
          user_type: "msa".into(), 
          refresh_token: ms.refresh_token.or_else(|| Some(refresh_token.to_string())), 
          expires: ms.expires_in + now + 86400, 
     })
}