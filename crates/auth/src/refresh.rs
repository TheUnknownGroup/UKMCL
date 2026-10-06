use anyhow::Result;
use crate::microsoft::refresh;
use config::main_conf::{Main, AccountConfig};
use folders::hub_fold::make_hub;

use std::time::{SystemTime, UNIX_EPOCH};

pub async fn refresh_start() -> Result<(), String> {
     let home = make_hub().map_err(|e| e.to_string());
     let main = home?.join("config.toml");
     
     let acc = Main::load(&main.to_string_lossy().to_string()).unwrap();

     let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

     if acc.account.user_type != "msa" {
          return Ok(())
     }

     if acc.account.expires > now + 300 {
          return Ok(())
     }

     let refresh_token = acc.account.refresh_token.clone().ok_or("No refresh token stored")?;

     match refresh(&refresh_token).await {
          Ok(ses) => {
               let conf = AccountConfig::new(&ses.username, &ses.uuid, &ses.access_token, &ses.user_type, &ses.refresh_token, ses.expires);
               Main::update_acc(&main.to_string_lossy().to_string(), conf).map_err(|e| e.to_string())?;
               Ok(())
          }
          Err(e) => {
               Err(format!("Session expired: {}", e))
          }
     }
}