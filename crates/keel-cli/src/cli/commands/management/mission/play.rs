//! Mission play command — play verification artifacts

use anyhow::{Result, anyhow};
use owo_colors::OwoColorize;
use std::path::Path;

use crate::cli::style;
use keel::infrastructure::loader::load_board;

/// Run the mission play command
pub fn run(id: Option<&str>) -> Result<()> {
    let board_dir = keel::infrastructure::config::find_board_dir()?;
    let board = load_board(&board_dir)?;

    if let Some(id) = id {
        let mission = board.require_mission(id)?;
        play_mission(mission)?;
    } else {
        // Play all verified missions in last-verified order
        let mut verified_missions: Vec<_> = board
            .missions
            .values()
            .filter(|m| {
                m.frontmatter.verified_at.is_some() && m.frontmatter.verification_artifact.is_some()
            })
            .collect();

        if verified_missions.is_empty() {
            println!("No verified missions with playback artifacts found.");
            return Ok(());
        }

        // Sort by verified_at
        verified_missions.sort_by_key(|m| m.frontmatter.verified_at);

        println!("🎬 Playing verified mission artifacts in completion order...");
        println!();

        for mission in verified_missions {
            play_mission(mission)?;
        }
    }

    Ok(())
}

fn play_mission(mission: &keel::domain::model::Mission) -> Result<()> {
    let artifact = mission
        .frontmatter
        .verification_artifact
        .as_ref()
        .ok_or_else(|| anyhow!("Mission {} has no verification artifact", mission.id()))?;

    let verified_at = mission
        .frontmatter
        .verified_at
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_else(|| "unknown".to_string());

    println!("────────────────────────────────────────────────────────────");
    println!(
        "{} {}",
        "Mission:".bold(),
        style::styled_mission_id(mission.id())
    );
    println!("{}  {}", "Title:".bold(), mission.title());
    println!("{}   {}", "Verified:".bold(), verified_at.dimmed());
    println!("{} {}", "Artifact:".bold(), artifact.cyan());
    println!("────────────────────────────────────────────────────────────");

    let mission_dir = mission.path.parent().unwrap();
    let artifact_path = mission_dir.join(artifact);

    if !artifact_path.exists() {
        println!(
            "{} Artifact file not found: {}",
            "Warning:".yellow(),
            artifact_path.display()
        );
        return Ok(());
    }

    // Attempt to play the artifact
    play_artifact(&artifact_path)?;

    println!();
    Ok(())
}

fn play_artifact(path: &Path) -> Result<()> {
    let path_str = path.to_string_lossy();

    // Prefer atext (Art as Text) for terminal-native high-dimension playback
    let env = atext::TerminalEnvironment::capture();
    let profile = atext::detect_terminal_profile(&env);

    if let Ok(text) = atext::render_to_text(path, &profile) {
        println!("{}", text);
        return Ok(());
    }

    // Fallback to ffplay for GIFs/Videos
    if is_media_file(&path_str)
        && let Ok(status) = std::process::Command::new("ffplay")
            .arg("-autoexit")
            .arg("-nodisp") // Don't show audio waves if it's just video
            .arg("-loglevel")
            .arg("quiet")
            .arg(path)
            .status()
        && status.success()
    {
        return Ok(());
    }

    // Fallback to system open
    #[cfg(target_os = "macos")]
    let opener = "open";
    #[cfg(target_os = "linux")]
    let opener = "xdg-open";
    #[cfg(target_os = "windows")]
    let opener = "start";

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        println!(
            "Playback not supported on this OS. Please open: {}",
            path.display()
        );
        return Ok(());
    }

    #[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
    {
        let mut cmd = std::process::Command::new(opener);
        #[cfg(target_os = "windows")]
        cmd.arg("/b"); // Use /b to not wait on windows start command

        if cmd.arg(path).status().is_ok() {
            println!("Opening artifact with system viewer...");
        } else {
            println!(
                "Failed to open artifact. Please open manually: {}",
                path.display()
            );
        }
    }

    Ok(())
}

fn is_media_file(path: &str) -> bool {
    let lower = path.to_lowercase();
    lower.ends_with(".gif")
        || lower.ends_with(".mp4")
        || lower.ends_with(".webm")
        || lower.ends_with(".mov")
}
