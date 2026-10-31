use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use utils::{client::base, folders::make_hub, toasts::{ToastKind, toast}};
use config::{account::{AccountConfig, AccountStruct}, auth_microsoft::AuthMSA};
use crate::{device_code::request_code, microsoft::poll_for_token, minecraft::{get_minecraft, get_profile}, refresh::refresh, xbox::get_xbox, xsts::get_xsts};

pub mod device_code;
pub mod microsoft;
pub mod xbox;
pub mod xsts;
pub mod refresh;
pub mod minecraft;

// pub const CLIENT_ID: &str = match option_env!("MC_CLIENT_ID") { Some(id) => id, None => "fallback-id" };
pub const CLIENT_ID: &str = "56bb5bc5-ab86-4960-bf94-20e638adc063";
const DEVICE_URL: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode";
const TOKEN_URL: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/token";
const XBOX_URL: &str = "https://user.auth.xboxlive.com/user/authenticate";
const XSTS_URL: &str = "https://xsts.auth.xboxlive.com/xsts/authorize";
const MC_AUTH: &str = "https://api.minecraftservices.com/authentication/login_with_xbox";
const MC_PROF: &str = "https://api.minecraftservices.com/minecraft/profile";


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Authenticated {
     pub username: String,
     pub uuid: String,
     pub access_token: String,
     pub user_type: String,
     pub refresh_token: Option<String>,
     pub expires: u64,
}

pub async fn auth(app: AppHandle) -> anyhow::Result<Authenticated> {
     let client = base();
     let home = make_hub()?;
     let main = home.join("msa.toml").to_string_lossy().to_string();
     let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
     let device = request_code(&app, &client).await?;
     AuthMSA::new(&device.verification_uri, &device.user_code, now + device.expires_in).save(&main)?;
     app.emit("open_msa", ())?;
     let mico_token = match poll_for_token(&app, &client, &device).await {
          Ok(t) => {
               toast(&app, ToastKind::Success, "Microsoft Auth is successful.");
               t
          },
          Err(e) => {
               app.emit("close_msa", ())?;
               toast(&app, ToastKind::Error, format!("{e:#}"));
               return Err(e);
          }
     };
     app.emit("close_msa", ())?;
     let result = async {
          let (xbox, uhs) = get_xbox(&app, &client, &mico_token.access_token).await?;
          let xsts = get_xsts(&app, &client, &xbox).await?;
          let mc = get_minecraft(&app, &client, &uhs, &xsts).await?;
          let profile = get_profile(&app, &client, &mc.access_token).await?;
          Ok::<_, anyhow::Error>(Authenticated {
               username: profile.name,
               uuid: profile.id,
               access_token: mc.access_token,
               user_type: "msa".into(),
               refresh_token: mico_token.refresh_token,
               expires: now + 86400,
          })
     }.await;

     AuthMSA::delete(&main)?;
     result
}

pub async fn start_refresh(app: AppHandle) -> anyhow::Result<(), String> {
     let home = make_hub().map_err(|e| e.to_string());
     let main = home?.join("account.toml");
     let acc = AccountStruct::load(&main.to_string_lossy().as_ref()).map_err(|e| format!("{}", e))?;
     let now = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| e.to_string())?.as_secs();
     if acc.account.user_type != "msa" || acc.account.expires > now + 300 {
          return Ok(())
     }
     let token = acc.account.refresh_token.clone().ok_or("No refresh token stored")?;
     let ses = refresh(&app, &token).await.map_err(|e| format!("{e:#}"))?;
     let conf = AccountConfig::new(&ses.username, &ses.uuid, &ses.access_token, &ses.user_type, &ses.refresh_token, ses.expires);
     AccountStruct::update_acc(&main.to_string_lossy().as_ref(), conf).map_err(|e| e.to_string())?;
     toast(&app, ToastKind::Info, "Refresh for minecraft auth was a success.");
     Ok(())
}
