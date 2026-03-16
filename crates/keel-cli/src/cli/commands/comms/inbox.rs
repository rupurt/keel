//! `keel inbox` command

use super::{PingMessage, inbox_dir};
use anyhow::Result;
use spoke_auth::ExecutionContext;
use std::fs;
use std::path::Path;

pub fn run(_ctx: &ExecutionContext, board_dir: &Path, json: bool) -> Result<()> {
    let dir = inbox_dir(board_dir);
    if !dir.exists() {
        if json {
            println!("[]");
        } else {
            println!("Inbox is empty.");
        }
        return Ok(());
    }

    let mut pings = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path)?;
            let ping: PingMessage = serde_json::from_str(&content)?;
            pings.push(ping);
        }
    }

    pings.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    if json {
        println!("{}", serde_json::to_string_pretty(&pings)?);
    } else if pings.is_empty() {
        println!("Inbox is empty.");
    } else {
        println!("{:<12} {:<20} {:<10} MESSAGE", "ID", "TIMESTAMP", "STATUS");
        println!("{}", "-".repeat(80));
        for ping in pings {
            let status = format!("{:?}", ping.status);
            println!(
                "{:<12} {:<20} {:<10} {}",
                ping.id,
                ping.timestamp.format("%Y-%m-%d %H:%M:%S"),
                status,
                ping.message
            );
        }
    }

    Ok(())
}
