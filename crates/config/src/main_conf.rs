use serde::{Deserialize, Serialize};
use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;

#[derive(Debug)]
pub struct AccountBuilder {
     username: String,
     uuid: String,
     access_token: String,
     user_type: String,
     max: u32,
     min: u32,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Main {
     pub account: AccountConfig,
     #[serde(default)]
     pub java: JavaConfig,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct AccountConfig {
     pub username: String,
     pub uuid: String,
     pub access_token: String,
     pub user_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JavaConfig {
     pub max: u32,
     pub min: u32,
}

impl Default for JavaConfig {
     fn default() -> Self {
          Self {
               min: 2048,
               max: 4096,
          }
     }
}

impl Main {
     pub fn save(&self, path: &str) -> Result<()> {
          let toml_str = toml::to_string_pretty(self).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
          std::fs::write(path, toml_str)
     }
     
     pub fn load(path: &str) -> Result<Self> {
          let data = std::fs::read_to_string(path)?;
          toml::from_str(&data).map_err(|e| Error::new(ErrorKind::InvalidData, e))
     }
     
     pub fn update_ram(path: &str, java: JavaConfig) -> Result<Self> {
          let mut cfg = Self::load(path)?;
          cfg.java = java;
          cfg.save(path)?;
          Ok(cfg)
     }
     
     pub fn update_acc(path: &str, account: AccountConfig) -> Result<Self> {
          let mut cfg = Self::load(path)?;
          cfg.account = account;
          cfg.save(path)?;
          Ok(cfg)
     }
}

impl Default for AccountBuilder {
     fn default() -> Self {
         Self::new()
     }
}

impl AccountConfig {
     pub fn new(username: &str, uuid: &str, access_token: &str, user_type: &str) -> Self {
          Self {
               username: username.into(),
               uuid: uuid.into(),
               access_token: access_token.into(),
               user_type: user_type.into(),
          }
     }
}

impl AccountBuilder {
     pub fn new() -> Self {
          Self {
               username: String::new(),
               uuid: String::new(),
               access_token: String::new(),
               user_type: String::new(),
               max: 32768,
               min: 512,
          }
     }

     pub fn account(mut self, username: &str, uuid: &str, access_token: &str, user_type: &str) -> Self {
          self.username = username.into();
          self.uuid = uuid.into();
          self.access_token = access_token.into();
          self.user_type = user_type.into();
          self
     }
     
     pub fn java(mut self, min: u32, max: u32) -> Self {
          self.max = max;
          self.min = min;
          self
     }

     pub fn build(self) -> Main {
          Main {
               account: AccountConfig { 
                    username: self.username, 
                    uuid: self.uuid, 
                    access_token: self.access_token, 
                    user_type: self.user_type,
               },
               java: JavaConfig {
                    max: self.max,
                    min: self.min,
               }
          }
     }

     pub fn build_save(self, path: &str) -> std::io::Result<Main> {
          let cfg = self.build();
          cfg.save(path)?;
          Ok(cfg)
     }
}

impl JavaConfig {
     pub fn new(min: u32, max: u32) -> Self {
          Self {
               min: min,
               max: max,
          }
     }
}