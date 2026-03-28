//! `keel poke` command

use super::{PingStatus, check_auto_pong, load_ping, save_ping};
use anyhow::Result;
use chrono::Utc;
use spoke_auth::ExecutionContext;
use std::path::Path;

pub fn run(
    _ctx: &ExecutionContext,
    board_dir: &Path,
    id: Option<&str>,
    manual_pong: Option<&str>,
    is_self: bool,
    json: bool,
) -> Result<()> {
    if id.is_none() || is_self {
        // Self-healing: auto-install git hooks if configured
        let (config, _) = keel::infrastructure::config::load_config();
        if config.workflow.auto_install_hooks {
            self_heal_hooks(board_dir);
        }

        let heartbeat = keel::read_model::heartbeat::project(board_dir, Utc::now());

        crate::cli::presentation::audio::play(crate::cli::presentation::audio::SoundEvent::Poke);

        if json {
            println!(
                "{}",
                serde_json::json!({
                    "status": "poked",
                    "message": "Heartbeat is derived from repository activity",
                    "source": heartbeat.source,
                    "dirty": heartbeat.dirty,
                    "note": manual_pong,
                })
            );
        } else {
            println!(
                "System poke acknowledged. Heartbeat is derived from repository activity; run `keel heartbeat` to inspect the current charge."
            );
        }
        return Ok(());
    }

    let id = id.unwrap();
    let mut ping = load_ping(board_dir, id)?;

    if ping.status == PingStatus::Ponged {
        if json {
            println!("{}", serde_json::to_string_pretty(&ping)?);
        } else {
            println!("[{}] {}", id, ping.pong_message.as_ref().unwrap());
        }
        return Ok(());
    }

    let response = manual_pong
        .map(|s| s.to_string())
        .or_else(|| check_auto_pong(&ping.message));

    if let Some(pong) = response {
        ping.status = PingStatus::Ponged;
        ping.pong_message = Some(pong.clone());
        save_ping(board_dir, &ping)?;

        // Also save to outbox
        let outbox = super::outbox_dir(board_dir);
        let _ = std::fs::create_dir_all(&outbox);
        let outbox_path = outbox.join(format!("{}.json", ping.id));
        let _ = serde_json::to_string_pretty(&ping).map(|json| std::fs::write(&outbox_path, json));
    }

    if json {
        println!("{}", serde_json::to_string_pretty(&ping)?);
    } else if ping.status == PingStatus::Ponged {
        println!("[{}] {}", ping.id, ping.pong_message.as_ref().unwrap());
    } else {
        println!("[{}]", ping.id);
    }

    Ok(())
}

/// Check if pacemaker git hooks are installed; install them if missing.
fn self_heal_hooks(board_dir: &Path) {
    // Resolve the repo root from the board dir (board_dir is typically <root>/.keel)
    let repo_root = board_dir.parent().unwrap_or(board_dir);
    let git_hooks = repo_root.join(".git/hooks");

    let pre_commit = git_hooks.join("pre-commit");
    let commit_msg = git_hooks.join("commit-msg");

    let needs_install = !pre_commit.exists()
        || !commit_msg.exists()
        || std::fs::read_to_string(&pre_commit)
            .map(|c| !c.contains("keel pacemaker protocol"))
            .unwrap_or(true)
        || std::fs::read_to_string(&commit_msg)
            .map(|c| !c.contains("keel pacemaker protocol"))
            .unwrap_or(true);

    if needs_install {
        match crate::cli::commands::setup::hooks::run_in(Some(repo_root)) {
            Ok(()) => {}
            Err(e) => eprintln!("Warning: auto-install hooks failed: {e}"),
        }
    }
}
