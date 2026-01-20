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
        let watched_path = path.canonicalize().unwrap_or_else(|_| path.clone());

        let (tx, rx) = channel();

        // Create debouncer with 100ms timeout
        let mut debouncer = new_debouncer(Duration::from_millis(100), None, tx)?;

        // Watch the parent directory for better compatibility across platforms
        // Watching a file directly doesn't always work, especially on macOS
        let watch_path = if watched_path.is_file() {
            watched_path.parent().unwrap_or(&watched_path)
        } else {
            &watched_path
        };

        debouncer
            .watcher()
            .watch(watch_path, RecursiveMode::NonRecursive)?;

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
                    // Filter events to only process our watched file
                    let is_our_file = event.paths.iter().any(|p| {
                        p == &self.watched_path ||
                        p.canonicalize().ok().as_ref() == Some(&self.watched_path)
                    });

                    if is_our_file {
                        match event.kind {
                            notify::EventKind::Modify(_) => was_modified = true,
                            notify::EventKind::Remove(_) => was_deleted = true,
                            notify::EventKind::Create(_) => was_created = true,
                            notify::EventKind::Access(_) => {
                                // Some systems generate access events for modifications
                                was_modified = true;
                            }
                            notify::EventKind::Any => {
                                // Treat generic events as modifications
                                was_modified = true;
                            }
                            _ => {}
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
