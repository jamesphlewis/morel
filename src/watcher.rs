use crate::error::Result;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use notify_debouncer_full::{new_debouncer, DebouncedEvent, Debouncer, FileIdMap};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

#[derive(Debug, PartialEq)]
pub enum FileEvent {
    NoChange,
    Modified,
    Deleted,
    Created,
}

pub struct FileWatcher {
    _debouncer: Debouncer<RecommendedWatcher, FileIdMap>,
    receiver: Receiver<std::result::Result<Vec<DebouncedEvent>, Vec<notify::Error>>>,
    watched_path: PathBuf,
}

impl FileWatcher {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let watched_path = path.clone();

        let (tx, rx) = channel();

        // Create debouncer with 100ms timeout
        let mut debouncer = new_debouncer(Duration::from_millis(100), None, tx)?;

        // Watch the specific file
        debouncer
            .watcher()
            .watch(&path, RecursiveMode::NonRecursive)?;

        Ok(Self {
            _debouncer: debouncer,
            receiver: rx,
            watched_path,
        })
    }

    pub fn check_for_changes(&mut self) -> Result<FileEvent> {
        // Non-blocking check for events
        match self.receiver.try_recv() {
            Ok(Ok(events)) => {
                // Process events to determine what happened
                let mut was_modified = false;
                let mut was_deleted = false;
                let mut was_created = false;

                for event in events {
                    for path in &event.paths {
                        if path == &self.watched_path {
                            match event.kind {
                                notify::EventKind::Modify(_) => was_modified = true,
                                notify::EventKind::Remove(_) => was_deleted = true,
                                notify::EventKind::Create(_) => was_created = true,
                                _ => {}
                            }
                        }
                    }
                }

                // Prioritize event types
                if was_deleted {
                    Ok(FileEvent::Deleted)
                } else if was_created {
                    Ok(FileEvent::Created)
                } else if was_modified {
                    Ok(FileEvent::Modified)
                } else {
                    Ok(FileEvent::NoChange)
                }
            }
            Ok(Err(_errors)) => {
                // Error in watching - treat as no change
                Ok(FileEvent::NoChange)
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                // No events
                Ok(FileEvent::NoChange)
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                // Channel disconnected - watcher is dead
                Ok(FileEvent::NoChange)
            }
        }
    }
}
