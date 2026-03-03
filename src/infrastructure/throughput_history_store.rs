//! Filesystem store for throughput history snapshots.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::read_model::throughput_history::ThroughputHistory;

// Keep the existing filename to avoid churn across existing boards.
const THROUGHPUT_HISTORY_FILE: &str = "flow_history.json";

/// Persist throughput history if contents differ from the existing file.
pub fn save_if_changed(board_dir: &Path, history: &ThroughputHistory) -> Result<()> {
    let path = history_path(board_dir);
    let serialized = format!("{}\n", serde_json::to_string_pretty(history)?);
    let existing = fs::read_to_string(&path).unwrap_or_default();
    if existing != serialized {
        fs::write(path, serialized)?;
    }
    Ok(())
}

fn history_path(board_dir: &Path) -> PathBuf {
    board_dir.join(THROUGHPUT_HISTORY_FILE)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_model::throughput_history::{
        THROUGHPUT_HISTORY_SCHEMA_VERSION, ThroughputHistory,
    };
    use chrono::NaiveDate;
    use tempfile::TempDir;

    #[test]
    fn save_if_changed_writes_history_file() {
        let temp = TempDir::new().unwrap();
        let board_dir = temp.path();
        let history = ThroughputHistory {
            schema_version: THROUGHPUT_HISTORY_SCHEMA_VERSION,
            weekly: vec![],
        };

        save_if_changed(board_dir, &history).unwrap();

        let path = board_dir.join(THROUGHPUT_HISTORY_FILE);
        assert!(path.exists());
        let content = fs::read_to_string(path).unwrap();
        assert!(content.contains("\"schema_version\": 1"));
    }

    #[test]
    fn save_if_changed_is_stable_for_identical_content() {
        let temp = TempDir::new().unwrap();
        let board_dir = temp.path();
        let history = ThroughputHistory {
            schema_version: THROUGHPUT_HISTORY_SCHEMA_VERSION,
            weekly: vec![
                crate::read_model::throughput_history::WeeklyThroughputBucket {
                    week_start: NaiveDate::from_ymd_opt(2026, 3, 2).unwrap(),
                    stories_done: 1,
                    voyages_done: 0,
                    cycle_min_hours: Some(1.0),
                    cycle_median_hours: Some(2.0),
                    cycle_max_hours: Some(3.0),
                    acceptance_wait_median_hours: Some(0.5),
                },
            ],
        };

        save_if_changed(board_dir, &history).unwrap();
        let first = fs::read_to_string(board_dir.join(THROUGHPUT_HISTORY_FILE)).unwrap();

        save_if_changed(board_dir, &history).unwrap();
        let second = fs::read_to_string(board_dir.join(THROUGHPUT_HISTORY_FILE)).unwrap();

        assert_eq!(first, second);
    }
}
