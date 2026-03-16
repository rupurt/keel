//! `keel outbox` command

use super::{PingMessage, outbox_dir};
use anyhow::Result;
use spoke_auth::ExecutionContext;
use std::fs;
use std::path::Path;

pub fn run(_ctx: &ExecutionContext, board_dir: &Path, json: bool) -> Result<()> {
    let dir = outbox_dir(board_dir);
    if !dir.exists() {
        if json {
            println!("[]");
        } else {
            println!("Outbox is empty.");
        }
        return Ok(());
    }

    let mut messages = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path)?;
            let ping: PingMessage = serde_json::from_str(&content)?;
            messages.push(ping);
        }
    }

    messages.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    if json {
        println!("{}", serde_json::to_string_pretty(&messages)?);
    } else if messages.is_empty() {
        println!("Outbox is empty.");
    } else {
        println!("{:<12} {:<20} {:<10} MESSAGE", "ID", "TIMESTAMP", "STATUS");
        println!("{}", "-".repeat(80));
        for msg in messages {
            let status = format!("{:?}", msg.status);
            println!(
                "{:<12} {:<20} {:<10} {}",
                msg.id,
                msg.timestamp.format("%Y-%m-%d %H:%M:%S"),
                status,
                msg.message
            );
        }
    }

    Ok(())
}
