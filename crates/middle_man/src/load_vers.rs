use get::versions_get::fetch;
use types::main_dir::make_main;

pub async fn load_ids() -> Result<(), Box<dyn std::error::Error>> {
     let mani = fetch().await?;

     let vers: Vec<String> = mani.versions.iter().map(|v| v.id.clone()).collect();
     let json_str = serde_json::to_string_pretty(&vers)?;
     let home = make_main();
     let json = home?.join("versions.json");
     std::fs::write(&json, &json_str)?;

     Ok(())
}