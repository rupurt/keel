//! Heartbeat - the system's pacemaker and energization state

use std::path::PathBuf;
use std::time::SystemTime;

/// Heartbeat tracks the system's energization state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heartbeat {
    /// Path to the heartbeat file (.keel/heartbeat)
    pub path: PathBuf,
    /// Last modification time of the file
    pub last_modified: SystemTime,
    /// Whether the file is dirty (uncommitted) in Git
    pub is_dirty: bool,
}

impl Heartbeat {
    pub fn new(path: PathBuf, last_modified: SystemTime, is_dirty: bool) -> Self {
        Self {
            path,
            last_modified,
            is_dirty,
        }
    }

    /// Whether the system is energized (recent heartbeat)
    pub fn is_energized(&self, max_decay_seconds: u64) -> bool {
        match self.last_modified.elapsed() {
            Ok(elapsed) => elapsed.as_secs() < max_decay_seconds,
            Err(_) => false,
        }
    }
}
