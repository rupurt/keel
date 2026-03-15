//! Comms commands - ping and poke the inbox

pub mod ping;
pub mod poke;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PingMessage {
    pub id: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub status: PingStatus,
    pub pong_message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PingStatus {
    Pending,
    Ponged,
}

pub fn inbox_dir(board_dir: &Path) -> PathBuf {
    board_dir.join("inbox")
}

pub fn save_ping(board_dir: &Path, ping: &PingMessage) -> Result<()> {
    let dir = inbox_dir(board_dir);
    fs::create_dir_all(&dir)?;
    let path = dir.join(format!("{}.json", ping.id));
    let json = serde_json::to_string_pretty(ping)?;
    fs::write(&path, json).with_context(|| format!("Failed to write ping to {}", path.display()))?;
    Ok(())
}

pub fn load_ping(board_dir: &Path, id: &str) -> Result<PingMessage> {
    let path = inbox_dir(board_dir).join(format!("{}.json", id));
    let content = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read ping from {}", path.display()))?;
    let ping = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse ping from {}", path.display()))?;
    Ok(ping)
}

pub fn check_auto_pong(message: &str) -> Option<String> {
    let lower = message.to_lowercase();
    let words: Vec<&str> = lower.split_whitespace().collect();
    
    if words.contains(&"ping") {
        Some("pong".to_string())
    } else if words.contains(&"hello") || words.contains(&"hi") {
        Some("Hello! I am keel. How can I help?".to_string())
    } else if words.contains(&"help") {
        Some("I am a workflow engine. Try running `keel doctor` or `keel flow`.".to_string())
    } else {
        None
    }
}
