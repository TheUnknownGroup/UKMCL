use notify::{Watcher, RecursiveMode, Event, EventKind, Result};
use notify::event::{ModifyKind, RenameMode};
use std::sync::mpsc::channel;
use tauri::{AppHandle, Emitter};
use create::create_hub::check_dir;

pub fn watch_dir(app_handle: AppHandle) -> Result<()> {
     let (tx, rx) = channel();

     let mut watcher = notify::recommended_watcher(move |res: Result<Event>| {
          if let Ok(event) = res {
               tx.send(event).ok();
          }
     })?;

     let inst_dir = check_dir()?;
     watcher.watch(&inst_dir, RecursiveMode::NonRecursive)?;

     std::thread::spawn(move || {
          let _watcher = watcher;

          for event in rx {
               let is_removal = match &event.kind {
                    EventKind::Remove(_) => true,
                    EventKind::Modify(ModifyKind::Name(RenameMode::From)) => true,
                    _ => false,
               };

               if is_removal {
                    let _ = app_handle.emit("instance-removed", ());
               }
          }
     });

     Ok(())
}