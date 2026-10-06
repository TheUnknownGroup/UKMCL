use crate::main_conf::AccountBuilder;

use crate::offline::offline;
use folders::hub_fold::make_hub;

pub async fn build() -> Result<(), Box<dyn std::error::Error>> {
     let main2 = make_hub()?;
     if main2.join("config.toml").exists() {
          return Ok(())
     }
     let main_conf = AccountBuilder::new().account("User", &offline("User").hyphenated().to_string(), "0", "legacy").java(2048, 4096).build();
     main_conf.save(&main2.join("config.toml").to_string_lossy().to_string())?;
     
     Ok(())
}