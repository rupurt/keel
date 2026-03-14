use anyhow::Result;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use super::types::DoctorReport;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CachedReport {
    pub hash: String,
    pub report: DoctorReport,
    pub last_poked_at: SystemTime,
}

pub fn cache_path(board_dir: &Path) -> PathBuf {
    board_dir.join("cache").join("doctor.json")
}

pub fn load_cache(board_dir: &Path, current_hash: &str) -> Option<(DoctorReport, SystemTime)> {
    let path = cache_path(board_dir);
    if !path.exists() {
        return None;
    }

    let data = fs::read_to_string(path).ok()?;
    let cached: CachedReport = serde_json::from_str(&data).ok()?;

    if cached.hash == current_hash {
        Some((cached.report, cached.last_poked_at))
    } else {
        None
    }
}

pub fn save_cache(board_dir: &Path, hash: String, report: DoctorReport) -> Result<()> {
    let path = cache_path(board_dir);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let cached = CachedReport {
        hash,
        report,
        last_poked_at: SystemTime::now(),
    };

    let data = serde_json::to_string_pretty(&cached)?;
    fs::write(path, data)?;
    // println!("Diagnostics cache saved ({})", hash);
    Ok(())
}

/// Calculate a fast hash of the entire board state.
pub fn calculate_board_hash(board_dir: &Path) -> Result<String> {
    let mut hasher = Sha256::new();

    // Strategy: only hash direct children metadata and the root itself.
    // This is much faster than a recursive walk and covers most 'pokes'.
    let dirs = ["stories", "epics", "missions", "bearings", "adrs", "routines"];
    
    // Hash root metadata
    if let Ok(metadata) = board_dir.metadata() {
        update_hasher_with_metadata(&mut hasher, board_dir, &metadata);
    }

    for dir_name in dirs {
        let dir_path = board_dir.join(dir_name);
        if !dir_path.exists() {
            continue;
        }

        if let Ok(metadata) = dir_path.metadata() {
            update_hasher_with_metadata(&mut hasher, &dir_path, &metadata);
        }

        // Hash immediate children metadata and key docs within them
        if let Ok(entries) = fs::read_dir(&dir_path) {
            for entry in entries.filter_map(|e| e.ok()) {
                let entry_path = entry.path();
                if let Ok(metadata) = entry.metadata() {
                    update_hasher_with_metadata(&mut hasher, &entry_path, &metadata);
                }

                if entry_path.is_dir() {
                    let key_docs = ["README.md", "PRD.md", "SRS.md", "SDD.md", "BRIEF.md", "EVIDENCE.md", "ASSESSMENT.md", "CHARTER.md", "LOG.md"];
                    for doc in key_docs {
                        let doc_path = entry_path.join(doc);
                        if let Ok(metadata) = doc_path.metadata() {
                            update_hasher_with_metadata(&mut hasher, &doc_path, &metadata);
                        }
                    }
                }
            }
        }
    }

    Ok(format!("{:x}", hasher.finalize()))
}

fn update_hasher_with_metadata(hasher: &mut Sha256, path: &Path, metadata: &fs::Metadata) {
    hasher.update(path.to_string_lossy().as_bytes());
    hasher.update(metadata.len().to_le_bytes());
    if let Ok(mtime) = metadata.modified() {
        if let Ok(duration) = mtime.duration_since(SystemTime::UNIX_EPOCH) {
            hasher.update(duration.as_secs().to_le_bytes());
            hasher.update(duration.subsec_nanos().to_le_bytes());
        }
    }
}
