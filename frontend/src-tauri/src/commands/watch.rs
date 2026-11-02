use std::sync::mpsc::channel;
use notify::{Event, EventKind, RecursiveMode, Result, event::{ModifyKind, RenameMode}, Watcher};
use tauri::{AppHandle, Emitter};
use utils::folders::make_hub;

pub fn watch_directory(app_handle: AppHandle) -> Result<()> {
     let (tx, rx) = channel();
     let mut watcher = notify::recommended_watcher(move |res: Result<Event>| {
          if let Ok(event) = res {
               tx.send(event).ok();
          }
     })?;
     
     let inst_dir = make_hub()?;
     watcher.watch(&inst_dir.join("instances"), RecursiveMode::NonRecursive)?;
     std::thread::spawn(move || {
          let _watcher = watcher;
          for event in rx {
               let is_removal = matches!(&event.kind, EventKind::Remove(_) | EventKind::Modify(ModifyKind::Name(RenameMode::From)));
               if is_removal {
                    let _ = app_handle.emit("instance-removed", ());
               }
          }
     });
     
     Ok(())
}