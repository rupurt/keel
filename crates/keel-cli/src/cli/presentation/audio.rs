//! Audio feedback for CLI state transitions.
//!
//! Plays terminal bells or platform-native sounds when significant
//! events occur. Never blocks, never fails — audio is best-effort.

/// Events that can trigger audio feedback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundEvent {
    /// Story submitted for verification
    StorySubmitted,
    /// Story completed (accepted / auto-accepted)
    StoryComplete,
    /// Voyage completed (all stories done)
    VoyageDone,
    /// Mission achieved
    MissionAchieved,
    /// Mission verified — major milestone
    MissionVerified,
    /// System poked (energy restored)
    Poke,
    /// Doctor found warnings
    DoctorWarning,
    /// Doctor found errors
    DoctorError,
}

impl SoundEvent {
    /// How many bells to ring for this event.
    fn bell_count(self) -> u8 {
        match self {
            Self::Poke | Self::StorySubmitted | Self::DoctorWarning => 1,
            Self::StoryComplete | Self::DoctorError => 1,
            Self::VoyageDone | Self::MissionAchieved => 2,
            Self::MissionVerified => 3,
        }
    }

    /// Config key for custom sound lookup.
    fn config_key(self) -> &'static str {
        match self {
            Self::StorySubmitted => "story_submitted",
            Self::StoryComplete => "story_complete",
            Self::VoyageDone => "voyage_done",
            Self::MissionAchieved => "mission_achieved",
            Self::MissionVerified => "mission_verified",
            Self::Poke => "poke",
            Self::DoctorWarning => "doctor_warning",
            Self::DoctorError => "doctor_error",
        }
    }
}

/// Play audio feedback for an event. Non-blocking, never fails.
pub fn play(event: SoundEvent) {
    let (config, _) = keel::infrastructure::config::load_config();
    if !config.audio.enabled {
        return;
    }

    // Check for a custom sound file first
    if let Some(path) = config.audio.sounds.get(event.config_key()) {
        if let Some(player) = find_platform_player() {
            let path = path.clone();
            let player = player.to_string();
            std::thread::spawn(move || {
                let _ = std::process::Command::new(&player).arg(&path).spawn();
            });
            return;
        }
    }

    // Fall back to terminal bell
    if config.audio.bell {
        let count = event.bell_count();
        std::thread::spawn(move || {
            play_bell(count);
        });
    }
}

fn play_bell(count: u8) {
    for i in 0..count {
        if i > 0 {
            std::thread::sleep(std::time::Duration::from_millis(150));
        }
        eprint!("\x07");
    }
}

fn find_platform_player() -> Option<&'static str> {
    #[cfg(target_os = "linux")]
    {
        for cmd in &["paplay", "aplay"] {
            if command_exists(cmd) {
                return Some(cmd);
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        if command_exists("afplay") {
            return Some("afplay");
        }
    }

    None
}

fn command_exists(cmd: &str) -> bool {
    std::process::Command::new("which")
        .arg(cmd)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bell_count_escalates_with_milestone_significance() {
        assert_eq!(SoundEvent::Poke.bell_count(), 1);
        assert_eq!(SoundEvent::StoryComplete.bell_count(), 1);
        assert_eq!(SoundEvent::VoyageDone.bell_count(), 2);
        assert_eq!(SoundEvent::MissionAchieved.bell_count(), 2);
        assert_eq!(SoundEvent::MissionVerified.bell_count(), 3);
    }

    #[test]
    fn config_keys_are_unique() {
        let events = [
            SoundEvent::StorySubmitted,
            SoundEvent::StoryComplete,
            SoundEvent::VoyageDone,
            SoundEvent::MissionAchieved,
            SoundEvent::MissionVerified,
            SoundEvent::Poke,
            SoundEvent::DoctorWarning,
            SoundEvent::DoctorError,
        ];
        let keys: Vec<_> = events.iter().map(|e| e.config_key()).collect();
        let unique: std::collections::HashSet<_> = keys.iter().collect();
        assert_eq!(keys.len(), unique.len(), "config keys must be unique");
    }
}
