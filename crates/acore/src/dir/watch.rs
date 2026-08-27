use notify::{Watcher, RecursiveMode, Event, EventKind};
use notify::event::{ModifyKind, RenameMode};
use std::sync::mpsc::channel;
use tauri::{AppHandle, Emitter};

use crate::dir::main_and_inst::check_dir;

pub fn watch_dir(app_handle: AppHandle) -> notify::Result<()> {
    let (tx, rx) = channel();

    let mut watcher = notify::recommended_watcher(move |res: notify::Result<Event>| {
        if let Ok(event) = res {
            tx.send(event).ok();
        }
    })?;

    let instance_dir = check_dir()?;
    
    watcher.watch(&instance_dir, RecursiveMode::NonRecursive)?;

    std::thread::spawn(move || {
        let _watcher = watcher;

        for event in rx {
            println!("[watcher] event: {:?}", event);

            let is_removal = match &event.kind {
                EventKind::Remove(_) => true,
                EventKind::Modify(ModifyKind::Name(RenameMode::From)) => true,
                _ => false,
            };
            
            if is_removal {
                println!("[watcher] emitting instance-removed");
                let _ = app_handle.emit("instance-removed", ());
            }
        }
    });

    Ok(())
}