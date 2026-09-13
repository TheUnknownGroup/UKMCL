use minecraft::req::fetch;
use folders::hub_fold::make_hub;

pub async fn make_json() -> Result<(), Box<dyn std::error::Error>> {
     let mani = fetch().await?;

     let all: Vec<String> = mani.versions.iter()
          .map(|v| v.id.clone()).collect();

     let home = make_hub()?.join("jsons");

     let pairs = [
          ("versions.json", &all),
     ];
     
     for (filename, bys) in pairs {
          let json_dir = home.join(filename);
          if !json_dir.exists() {
               let file = serde_json::to_string_pretty(bys)?;
               std::fs::write(&json_dir, file)?;
          } else {
               let raw = std::fs::read_to_string(&json_dir)?;
               let existing: Vec<String> = serde_json::from_str(&raw)?;
               if existing != *bys {
                    let file = serde_json::to_string_pretty(bys)?;
                    std::fs::write(&json_dir, file)?;
               }
          }
     }
     
     Ok(())
}