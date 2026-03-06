use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use crate::infrastructure::verification::executor::ExecuteResult;
use wait_timeout::ChildExt;

pub fn run_tape(
    working_dir: &Path,
    tape_path: &Path,
    outputs: &[PathBuf],
) -> Result<ExecuteResult> {
    if !tape_path.exists() {
        return Ok(ExecuteResult {
            exit_code: 1,
            stdout: String::new(),
            stderr: format!("Tape file not found: {}", tape_path.display()),
            error: Some("TapeNotFound".to_string()),
        });
    }

    for output in outputs {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent)?;
        }
    }

    if std::env::var("KEEL_MOCK_VHS").is_ok() {
        for output in outputs {
            fs::write(output, b"mock vhs output")?;
        }
        return Ok(ExecuteResult {
            exit_code: 0,
            stdout: format!("mocked vhs run for {}", tape_path.display()),
            stderr: String::new(),
            error: None,
        });
    }

    let mut command = Command::new("vhs");
    command.arg(tape_path);
    for output in outputs {
        command.arg("-o").arg(output);
    }
    let mut child = command
        .current_dir(working_dir)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    let output = match child.wait_timeout(Duration::from_secs(30))? {
        Some(_) => child.wait_with_output()?,
        None => {
            child.kill()?;
            return Ok(ExecuteResult {
                exit_code: 1,
                stdout: String::new(),
                stderr: format!("VHS timed out for {}", tape_path.display()),
                error: Some("Timeout".to_string()),
            });
        }
    };

    Ok(ExecuteResult {
        exit_code: output.status.code().unwrap_or(1),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        error: None,
    })
}
