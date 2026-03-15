//! Generic frontmatter update for state transitions.

use std::fmt::Display;

use anyhow::Result;
use chrono::Local;

use super::TimestampUpdates;

/// Update frontmatter with new status and timestamps.
///
/// This generic function replaces nearly-identical implementations
/// across transition and voyage commands. It handles:
///
/// - Updating the `status` field to the new value
/// - Updating `updated_at` to current datetime (if configured)
/// - Updating `submitted_at` to current datetime (if configured)
/// - Updating `completed_at` to current datetime (if configured)
/// - Inserting missing timestamp fields before closing `---`
///
/// The status parameter accepts any type implementing Display,
/// including canonical story and voyage state enums.
pub fn update_frontmatter(
    content: &str,
    new_status: impl Display,
    timestamps: &TimestampUpdates,
) -> Result<String> {
    let now_dt = Local::now();
    let now = now_dt.format("%Y-%m-%dT%H:%M:%S").to_string();

    let mut result = String::new();
    let mut in_frontmatter = false;
    let mut delimiter_count = 0;

    // Track which fields we've updated
    let mut updated_updated_at = false;
    let mut updated_submitted_at = false;
    let mut updated_completed_at = false;

    for line in content.lines() {
        if line == "---" {
            delimiter_count += 1;
            in_frontmatter = delimiter_count == 1;

            // Before closing `---`, insert any missing timestamp fields
            if delimiter_count == 2 {
                if timestamps.updated_at && !updated_updated_at {
                    result.push_str(&format!("updated_at: {}\n", now));
                }
                if timestamps.submitted_at && !updated_submitted_at {
                    result.push_str(&format!("submitted_at: {}\n", now));
                }
                if timestamps.completed_at && !updated_completed_at {
                    result.push_str(&format!("completed_at: {}\n", now));
                }
            }
        }

        if in_frontmatter {
            if line.starts_with("status:") {
                // Update status to new value
                result.push_str(&format!("status: {}\n", new_status));
            } else if line.starts_with("updated_at:") && timestamps.updated_at {
                // Update updated_at (datetime format)
                result.push_str(&format!("updated_at: {}\n", now));
                updated_updated_at = true;
            } else if line.starts_with("submitted_at:") && timestamps.submitted_at {
                // Update submitted_at (datetime format)
                result.push_str(&format!("submitted_at: {}\n", now));
                updated_submitted_at = true;
            } else if (line.starts_with("completed:") || line.starts_with("completed_at:"))
                && timestamps.completed_at
            {
                // Update completed_at (datetime format)
                result.push_str(&format!("completed_at: {}\n", now));
                updated_completed_at = true;
            } else {
                result.push_str(line);
                result.push('\n');
            }
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use regex::Regex;

    fn sample_frontmatter() -> &'static str {
        r#"---
id: FEAT0001
title: Test Story
type: feat
status: backlog
priority: high
---
Body content
"#
    }

    fn frontmatter_with_dates() -> &'static str {
        r#"---
id: FEAT0001
title: Test Story
type: feat
status: in-progress
priority: high
created_at: 2026-01-01
updated_at: 2026-01-15
---
Body content
"#
    }

    #[test]
    fn start_transition_updates_status() {
        let timestamps = TimestampUpdates::updated_only();
        let result =
            update_frontmatter(sample_frontmatter(), StoryState::InProgress, &timestamps).unwrap();

        assert!(result.contains("status: in-progress"));
        assert!(!result.contains("status: backlog"));
    }

    #[test]
    fn start_transition_adds_updated_at() {
        let timestamps = TimestampUpdates::updated_only();
        let result =
            update_frontmatter(sample_frontmatter(), StoryState::InProgress, &timestamps).unwrap();

        assert!(result.contains("updated_at: "));
        assert!(!result.contains("started_at: "));
    }

    #[test]
    fn start_transition_updates_existing_updated_at() {
        let timestamps = TimestampUpdates::updated_only();
        let result = update_frontmatter(
            frontmatter_with_dates(),
            StoryState::InProgress,
            &timestamps,
        )
        .unwrap();

        // Should update existing date, not duplicate
        let count = result.matches("updated_at:").count();
        assert_eq!(count, 1);
    }

    #[test]
    fn submit_transition_adds_submitted_at() {
        let timestamps = TimestampUpdates::with_submitted();
        let result = update_frontmatter(
            frontmatter_with_dates(),
            StoryState::NeedsHumanVerification,
            &timestamps,
        )
        .unwrap();

        assert!(result.contains("status: needs-human-verification"));
        assert!(result.contains("submitted_at: "));
        assert!(result.contains("updated_at: "));
        let submitted_re =
            Regex::new(r"submitted_at: \d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}").unwrap();
        assert!(
            submitted_re.is_match(&result),
            "submitted_at should be datetime: {result}"
        );
    }

    #[test]
    fn accept_transition_adds_completed_at() {
        let timestamps = TimestampUpdates::with_completed();
        let result =
            update_frontmatter(frontmatter_with_dates(), StoryState::Done, &timestamps).unwrap();

        assert!(result.contains("status: done"));
        assert!(result.contains("completed_at: "));
        assert!(result.contains("updated_at: "));
        let completed_re =
            Regex::new(r"completed_at: \d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}").unwrap();
        assert!(
            completed_re.is_match(&result),
            "completed_at should be datetime: {result}"
        );
    }

    #[test]
    fn reject_transition_only_updates_status_and_updated() {
        let timestamps = TimestampUpdates::updated_only();
        let result =
            update_frontmatter(frontmatter_with_dates(), StoryState::Rejected, &timestamps)
                .unwrap();

        assert!(result.contains("status: rejected"));
        assert!(result.contains("updated_at: "));
        assert!(!result.contains("submitted_at:"));
        assert!(!result.contains("completed_at:"));
    }

    #[test]
    fn ice_transition_updates_to_icebox() {
        let timestamps = TimestampUpdates::updated_only();
        let result =
            update_frontmatter(sample_frontmatter(), StoryState::Icebox, &timestamps).unwrap();

        assert!(result.contains("status: icebox"));
    }

    #[test]
    fn thaw_transition_updates_to_backlog() {
        let content = r#"---
id: FEAT0001
title: Test Story
type: feat
status: icebox
---
Body
"#;
        let timestamps = TimestampUpdates::updated_only();
        let result = update_frontmatter(content, StoryState::Backlog, &timestamps).unwrap();

        assert!(result.contains("status: backlog"));
    }

    #[test]
    fn preserves_body_content() {
        let timestamps = TimestampUpdates::updated_only();
        let result =
            update_frontmatter(sample_frontmatter(), StoryState::InProgress, &timestamps).unwrap();

        assert!(result.contains("Body content"));
    }

    #[test]
    fn preserves_other_frontmatter_fields() {
        let timestamps = TimestampUpdates::updated_only();
        let result =
            update_frontmatter(sample_frontmatter(), StoryState::InProgress, &timestamps).unwrap();

        assert!(result.contains("id: FEAT0001"));
        assert!(result.contains("title: Test Story"));
        assert!(result.contains("priority: high"));
    }
}
