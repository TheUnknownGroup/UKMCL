use serde::{Serialize, Deserialize};
use std::io::{Error, ErrorKind, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthMSA {
     pub verification_uri: String,
     pub user_code: String,
     pub expires_in: u64,
}

impl AuthMSA {
     pub fn new(verify: &str, user_code: &str, expires: u64) -> Self {
          Self {
               verification_uri: verify.into(),
               user_code: user_code.into(),
               expires_in: expires,
          }
     }

     pub fn save(&self, path: &str) -> Result<()> {
          let toml_str = toml::to_string_pretty(self).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
          std::fs::write(path, toml_str)
     }

     pub fn load(path: &str) -> Result<Self> {
          let data = std::fs::read_to_string(path)?;
          toml::from_str(&data).map_err(|e| Error::new(ErrorKind::InvalidData, e))
     }

     pub fn delete(path: &str) -> Result<()> {
          std::fs::remove_file(path)
     }
}