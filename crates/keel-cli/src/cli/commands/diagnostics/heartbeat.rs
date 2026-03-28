//! Heartbeat command - inspect derived repository activity.

use std::path::Path;

use anyhow::Result;
use chrono::{SecondsFormat, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct HeartbeatOutput {
    state: &'static str,
    source: keel::read_model::heartbeat::HeartbeatSource,
    energized: bool,
    dirty: bool,
    dirty_files: usize,
    latest_path: Option<String>,
    last_activity_at: String,
    age_seconds: i64,
    decay_minutes: u64,
}

pub fn run(board_dir: &Path, json: bool) -> Result<()> {
    let now = Utc::now();
    let heartbeat = keel::read_model::heartbeat::project(board_dir, now);
    let (config, _) = keel::infrastructure::config::load_config();
    let decay_minutes = u64::from(config.workflow.battery_decay_minutes);
    let energized = heartbeat.is_energized(now, decay_minutes);
    let output = HeartbeatOutput {
        state: if energized { "energized" } else { "idle" },
        source: heartbeat.source,
        energized,
        dirty: heartbeat.dirty,
        dirty_files: heartbeat.dirty_paths.len(),
        latest_path: heartbeat
            .latest_path
            .as_ref()
            .map(|path| path.display().to_string()),
        last_activity_at: heartbeat
            .last_activity_at
            .to_rfc3339_opts(SecondsFormat::Secs, true),
        age_seconds: heartbeat.age_seconds(now),
        decay_minutes,
    };

    if json {
        println!("{}", serde_json::to_string_pretty(&output)?);
        return Ok(());
    }

    println!("Heartbeat");
    println!("  State: {}", output.state);
    println!("  Source: {:?}", output.source);
    println!("  Last Activity: {}", output.last_activity_at);
    println!("  Age: {}", render_age(output.age_seconds));
    if let Some(path) = &output.latest_path {
        println!("  Latest Path: {}", path);
    }
    if output.dirty {
        println!("  Dirty Files: {}", output.dirty_files);
    }
    println!("  Decay Window: {}m", output.decay_minutes);
    Ok(())
}

fn render_age(age_seconds: i64) -> String {
    if age_seconds < 60 {
        return format!("{age_seconds}s");
    }
    if age_seconds < 3600 {
        return format!("{}m {}s", age_seconds / 60, age_seconds % 60);
    }
    format!("{}h {}m", age_seconds / 3600, (age_seconds % 3600) / 60)
}
