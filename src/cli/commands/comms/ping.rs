//! `keel ping` command

use super::{PingMessage, PingStatus, check_auto_pong, save_ping};
use anyhow::Result;
use chrono::Utc;
use std::path::Path;
use keel::infrastructure::story_id::generate_story_id;

pub fn run(board_dir: &Path, message: &str) -> Result<()> {
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
        println!("{}", pong);
    } else {
        println!("{}", id); // If no response, print the ID so the user knows what to poke
    }

    save_ping(board_dir, &ping)?;
    Ok(())
}
