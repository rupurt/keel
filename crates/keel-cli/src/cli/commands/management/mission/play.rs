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
    play_artifact(&artifact_path, mission.id())?;

    println!();
    Ok(())
}

fn play_artifact(path: &Path, mission_id: &str) -> Result<()> {
    let path_str = path.to_string_lossy();

    // Prefer atxt (Art as Text) for terminal-native high-dimension playback
    let env = atxt::TerminalEnvironment::capture();
    let terminal = atxt::detect_terminal_profile(&env);
    let probe = atxt::probe_path(path);
    let plan = atxt::plan_render(&probe, &terminal);

    if plan.output == atxt::OutputKind::FrameSequence {
        return run_high_fidelity_playback(path, &probe, &plan, mission_id);
    }

    if let Ok(text) = atxt::render_to_text(path, &terminal) {
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

fn run_high_fidelity_playback(
    path: &Path,
    probe: &atxt::ProbeResult,
    init_plan: &atxt::RenderPlan,
    mission_id: &str,
) -> Result<()> {
    use crossterm::event::{self, Event, KeyCode};
    use std::io::{Write, stdout};
    use std::thread;
    use std::time::Duration;
    use txt_scene::TheaterFrame;

    let sequence = atxt::decode_timed_sequence(path, probe, None)
        .map_err(|e| anyhow!("Failed to decode timed sequence: {}", e))?;
    let mut stdout = stdout();

    let mut current_plan = init_plan.clone();
    let mut theater = None;
    let mut previous_frame_output: Option<String> = None;
    let mut previous_timestamp_ms = 0;
    let mut needs_full_redraw = true;

    // Clear screen and hide cursor
    write!(stdout, "\x1b[2J\x1b[H\x1b[?25l").map_err(|e| anyhow!("Terminal IO error: {}", e))?;
    stdout
        .flush()
        .map_err(|e| anyhow!("Terminal IO error: {}", e))?;

    for (index, sample) in sequence.samples().iter().enumerate() {
        // Handle input and resize events
        while event::poll(Duration::ZERO).unwrap_or(false) {
            match event::read().unwrap() {
                Event::Key(key) => {
                    if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                        write!(stdout, "\x1b[?25h").ok();
                        return Ok(());
                    }
                }
                Event::Resize(_, _) => {
                    // Re-plan on resize
                    let env = atxt::TerminalEnvironment::capture();
                    let terminal = atxt::detect_terminal_profile(&env);
                    current_plan = atxt::plan_render(probe, &terminal);
                    needs_full_redraw = true;
                    previous_frame_output = None;
                    theater = None;

                    // Clear screen for clean resize
                    write!(stdout, "\x1b[2J\x1b[H").ok();
                }
                _ => {}
            }
        }

        if index > 0 {
            let delay_ms = sample.timestamp_ms.saturating_sub(previous_timestamp_ms);
            thread::sleep(Duration::from_millis(delay_ms));
        }

        // Prepare a single-frame plan for the individual frame
        let mut frame_plan = current_plan.clone();
        frame_plan.output = atxt::OutputKind::SingleFrame;

        let raw_frame = atxt::render_still_image(&sample.frame, &frame_plan)
            .map_err(|e| anyhow!("Rendering error: {}", e))?;

        // Initialize theater frame if needed (first frame or after resize)
        if theater.is_none() {
            let frame_width = raw_frame
                .lines()
                .map(txt_scene::visible_width)
                .max()
                .unwrap_or(0);
            theater = Some(TheaterFrame::new(frame_width));
        }

        let theater_frame = theater.as_ref().unwrap();

        if needs_full_redraw {
            // Print top border with mission ID
            let title = format!("MISSION {}", mission_id);
            write!(stdout, "\x1b[H{}", theater_frame.top_border(Some(&title)))
                .map_err(|e| anyhow!("Terminal IO error: {}", e))?;
            needs_full_redraw = false;
        }

        // Render each row inside the theater frame
        let mut frame_output = String::new();
        for line in raw_frame.lines() {
            frame_output.push_str(&theater_frame.row(line));
            frame_output.push('\n');
        }

        // Apply delta encoding if we have a previous frame
        match &previous_frame_output {
            Some(prev) if prev.len() == frame_output.len() => {
                let line_offset = 2; // Accounting for top border
                let mut line_index = line_offset;
                let mut col_index = 1;
                for (p, c) in prev.chars().zip(frame_output.chars()) {
                    if p != c {
                        write!(stdout, "\x1b[{};{}H{}", line_index, col_index, c)
                            .map_err(|e| anyhow!("Terminal IO error: {}", e))?;
                    }
                    if c == '\n' {
                        line_index += 1;
                        col_index = 1;
                    } else {
                        col_index += 1;
                    }
                }
            }
            _ => {
                // Redraw full frame starting below top border
                write!(stdout, "\x1b[2;1H{}", frame_output)
                    .map_err(|e| anyhow!("Terminal IO error: {}", e))?;
            }
        }

        stdout
            .flush()
            .map_err(|e| anyhow!("Terminal IO error: {}", e))?;

        previous_timestamp_ms = sample.timestamp_ms;
        previous_frame_output = Some(frame_output);
    }

    // Print bottom border
    if let Some(theater_frame) = theater {
        writeln!(stdout, "{}", theater_frame.bottom_border())
            .map_err(|e| anyhow!("Terminal IO error: {}", e))?;
    }

    // Show cursor and move to next line
    write!(stdout, "\x1b[?25h").map_err(|e| anyhow!("Terminal IO error: {}", e))?;
    stdout
        .flush()
        .map_err(|e| anyhow!("Terminal IO error: {}", e))?;
    println!();

    Ok(())
}

fn is_media_file(path: &str) -> bool {
    let lower = path.to_lowercase();
    lower.ends_with(".gif")
        || lower.ends_with(".mp4")
        || lower.ends_with(".webm")
        || lower.ends_with(".mov")
}
