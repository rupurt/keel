//! `keel ping` command

use super::{PingMessage, PingStatus, check_auto_pong, save_ping};
use anyhow::Result;
use chrono::Utc;
use std::fs;
use std::path::Path;
use keel::infrastructure::story_id::generate_story_id;
use spoke_auth::ExecutionContext;

pub fn run(_ctx: &ExecutionContext, board_dir: &Path, message: &str, json: bool) -> Result<()> {
    let id = generate_story_id(); // Reuse ID generation for standard ID format
    let mut ping = PingMessage {
        id: id.clone(),
        message: message.to_string(),
        timestamp: Utc::now(),
        status: PingStatus::Pending,
        pong_message: None,
    };

    if let Some(pong) = check_auto_pong(message) {
        ping.status = PingStatus::Ponged;
        ping.pong_message = Some(pong.clone());
    }

    save_ping(board_dir, &ping)?;
    
    // Also save to outbox
    let outbox = super::outbox_dir(board_dir);
    fs::create_dir_all(&outbox)?;
    let outbox_path = outbox.join(format!("{}.json", ping.id));
    let json_content = serde_json::to_string_pretty(&ping)?;
    fs::write(&outbox_path, json_content)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&ping)?);
    } else {
        if ping.status == PingStatus::Ponged {
            println!("[{}] {}", ping.id, ping.pong_message.as_ref().unwrap());
        } else {
            println!("[{}]", ping.id);
        }
    }

    Ok(())
}
