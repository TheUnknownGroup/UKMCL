use minecraft::req::fetch;
use folders::hub_fold::make_hub;

pub async fn make_json() -> Result<(), Box<dyn std::error::Error>> {
     let mani = fetch().await?;

     let all: Vec<String> = mani.versions.iter()
          .map(|v| v.id.clone()).collect();
     
     let real: Vec<String> = mani.versions.iter()
          .filter(|v| v.ver_type == "release")
          .map(|v| v.id.clone()).collect();
     
     let snap: Vec<String> = mani.versions.iter()
          .filter(|v| v.ver_type == "snapshot")
          .map(|v| v.id.clone()).collect();

     let old: Vec<String> = mani.versions.iter()
          .filter(|v| v.ver_type == "old_beta" || v.ver_type == "old_alpha")
          .map(|v| v.id.clone()).collect();

     let home = make_hub()?.join("jsons");

     let pairs = [
          ("versions.json", &all),
          ("releases.json", &real),
          ("snapshots.json", &snap),
          ("old.json", &old),
     ];
     
     for (filename, bys) in pairs {
          let json_dir = home.join(filename);
          if !json_dir.exists() {
               let file = serde_json::to_string_pretty(bys)?;
               std::fs::write(&json_dir, file)?;
          }
     }
     
     Ok(())
}