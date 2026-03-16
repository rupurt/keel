//! Notification utilities for Keel CLI

use anyhow::Result;
use std::process::Command;

/// Execute the configured notification command.
pub fn trigger_notification(command_str: &str) -> Result<()> {
    #[cfg(unix)]
    {
        Command::new("sh").arg("-c").arg(command_str).spawn()?;
    }
    #[cfg(windows)]
    {
        Command::new("cmd").arg("/C").arg(command_str).spawn()?;
    }
    Ok(())
}
