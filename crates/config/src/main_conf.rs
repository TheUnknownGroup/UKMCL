use serde::{Deserialize, Serialize};
use auth::offline::offline;

#[derive(Debug)]
pub struct AccountBuilder {
     username: String,
     uuid: String,
     access_token: String,
     user_type: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Main {
     pub main: AccountConfig,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AccountConfig {
     pub username: String,
     pub uuid: String,
     pub access_token: String,
     pub user_type: String,
}

impl Main {
     pub fn save(&self, path: &str) -> std::io::Result<()> {
          let toml_str = toml::to_string_pretty(self).unwrap();
          std::fs::write(path, toml_str)
     }
     
     pub fn load(path: &str) -> std::io::Result<Self> {
          let data = std::fs::read_to_string(path)?;
          let config: Self = toml::from_str(&data).expect("Failed to parse");
          Ok(config)
     }
}

impl AccountBuilder {
     pub fn new() -> Self {
          Self {
               username: String::new(),
               uuid: String::new(),
               access_token: String::new(),
               user_type: String::new(),
          }
     }

     pub fn account(mut self, username: &str, access_token: &str, user_type: &str) -> Self {
          self.username = username.into();
          self.uuid = offline(username).hyphenated().to_string();
          self.access_token = access_token.into();
          self.user_type = user_type.into();
          self
     }

     pub fn build(self) -> Main {
          Main {
               main: AccountConfig { 
                    username: self.username, 
                    uuid: self.uuid, 
                    access_token: self.access_token, 
                    user_type: self.user_type,
               }
          }
     }

     pub fn save(self, path: &str) -> std::io::Result<Main> {
          let cfg = self.build();
          cfg.save(path)?;
          Ok(cfg)
     }
}