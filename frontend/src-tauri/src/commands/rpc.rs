use discord_rich_presence::{DiscordIpc, DiscordIpcClient, activity::{self, ActivityType, Assets, Button}};
use std::sync::Mutex;
use tauri::{AppHandle, State, Manager};

pub struct DiscordRpc(pub Mutex<Option<DiscordIpcClient>>);

#[allow(unused)]
pub fn bot(app_handle: AppHandle) -> Result<(), String> {
     let state: State<DiscordRpc> = app_handle.state();
     let mut guard = state.0.lock().map_err(|e| e.to_string())?;
     if guard.is_none() {
          let mut client = DiscordIpcClient::new("1555224822408019988");
          client.connect().map_err(|e| e.to_string())?;
          *guard = Some(client);
     }

     if let Some(client) = guard.as_mut() {
          let icon = Assets::new();
          let image = icon.large_image("https://avatars.githubusercontent.com/u/171517439?s=400&v=4").large_text("UKMCL");
          
          let payload = activity::Activity::new()
            .name("UKMCL")
            .details("Playing Minecraft")
            .activity_type(ActivityType::Watching)
            .assets(image)
            .buttons(vec![
                 Button::new("Get UKMCL!", "https://github.com/TheUnknownGroup/UKMCL/releases/latest")
            ])
            .state_url("https://github.com/TheUnknownGroup/UKMCL");
          client.set_activity(payload).map_err(|e| e.to_string())?;
     }

     Ok(())
}