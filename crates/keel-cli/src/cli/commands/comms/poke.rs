//! `keel poke` command

use super::{PingStatus, check_auto_pong, load_ping, save_ping};
use anyhow::Result;
use std::path::Path;

pub fn run(board_dir: &Path, id: &str, manual_pong: Option<&str>, json: bool) -> Result<()> {
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
    }

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
