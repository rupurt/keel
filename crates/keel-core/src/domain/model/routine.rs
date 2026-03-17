//! Routine data structure
//!
//! Routines are recurring-work blueprints stored as a single markdown bundle
//! under `.keel/routines/<id>/README.md`.

use std::path::{Path, PathBuf};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use super::{Entity, deserialize_strict_datetime};

/// Opaque cadence metadata for a routine contract.
pub type RoutineCadence = Value;

/// Routine frontmatter from YAML.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RoutineFrontmatter {
    pub id: String,
    pub title: String,
    pub cadence: RoutineCadence,
    #[serde(rename = "target-scope")]
    pub target_scope: String,
    #[serde(
        default,
        deserialize_with = "deserialize_strict_datetime",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at: Option<NaiveDateTime>,
    #[serde(
        default,
        deserialize_with = "deserialize_strict_datetime",
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_at: Option<NaiveDateTime>,
    #[serde(
        default,
        rename = "operator-signal",
        skip_serializing_if = "Option::is_none"
    )]
    pub operator_signal: Option<String>,
    /// IDs of epics, voyages, or missions this routine has previously contributed to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lineage: Vec<String>,
}

/// A routine bundle with its frontmatter, authored blueprint body, and file path.
#[derive(Debug, Clone)]
pub struct Routine {
    pub frontmatter: RoutineFrontmatter,
    pub blueprint_markdown: String,
    pub path: PathBuf,
}

impl Routine {
    /// Construct a routine from parsed frontmatter, blueprint body, and README path.
    pub fn new(
        frontmatter: RoutineFrontmatter,
        blueprint_markdown: impl Into<String>,
        path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            frontmatter,
            blueprint_markdown: blueprint_markdown.into(),
            path: path.into(),
        }
    }

    pub fn id(&self) -> &str {
        &self.frontmatter.id
    }

    pub fn title(&self) -> &str {
        &self.frontmatter.title
    }

    pub fn cadence(&self) -> &RoutineCadence {
        &self.frontmatter.cadence
    }

    pub fn target_scope(&self) -> &str {
        &self.frontmatter.target_scope
    }

    pub fn blueprint_markdown(&self) -> &str {
        &self.blueprint_markdown
    }

    pub fn lineage(&self) -> &[String] {
        &self.frontmatter.lineage
    }
}

impl Entity for Routine {
    fn id(&self) -> &str {
        &self.frontmatter.id
    }

    fn title(&self) -> &str {
        &self.frontmatter.title
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::{Story, StoryFrontmatter, StoryState};
    use crate::infrastructure::parser::parse_frontmatter;

    #[test]
    fn routine_frontmatter_deserializes_canonical_contract() {
        let markdown = r#"---
id: routine-weekly-review
title: Weekly Review
cadence:
  cron: 0 9 * * 1
  timezone: America/Los_Angeles
  jitter-minutes: 15
target-scope: VDakm8eVW/VDcFd11nc
created_at: 2026-03-11T09:00:00
updated_at: 2026-03-11T09:30:00
---

# Blueprint

- Review the open backlog.
- Triage aging items.
"#;

        let (frontmatter, body): (RoutineFrontmatter, &str) = parse_frontmatter(markdown).unwrap();
        let routine = Routine::new(
            frontmatter.clone(),
            body.trim().to_string(),
            ".keel/routines/routine-weekly-review/README.md",
        );

        assert_eq!(frontmatter.id, "routine-weekly-review");
        assert_eq!(frontmatter.title, "Weekly Review");
        let cadence = frontmatter.cadence.as_mapping().unwrap();
        let cron_key = Value::from("cron");
        let timezone_key = Value::from("timezone");
        let jitter_key = Value::from("jitter-minutes");
        assert_eq!(cadence.get(&cron_key).unwrap(), "0 9 * * 1");
        assert_eq!(cadence.get(&timezone_key).unwrap(), "America/Los_Angeles");
        assert_eq!(cadence.get(&jitter_key).unwrap(), 15);
        assert_eq!(frontmatter.target_scope, "VDakm8eVW/VDcFd11nc");
        assert!(frontmatter.created_at.is_some());
        assert!(frontmatter.updated_at.is_some());
        assert_eq!(routine.id(), "routine-weekly-review");
        assert_eq!(routine.title(), "Weekly Review");
        assert_eq!(routine.cadence(), &frontmatter.cadence);
        assert_eq!(routine.target_scope(), "VDakm8eVW/VDcFd11nc");
        assert!(routine.blueprint_markdown().contains("# Blueprint"));
        assert!(
            routine
                .blueprint_markdown()
                .contains("Review the open backlog.")
        );
    }

    #[test]
    fn routine_frontmatter_rejects_unknown_fields() {
        let yaml = r#"
id: routine-weekly-review
title: Weekly Review
cadence:
  cron: 0 9 * * 1
target-scope: VDakm8eVW/VDcFd11nc
unexpected: true
"#;

        let result: Result<RoutineFrontmatter, _> = serde_yaml::from_str(yaml);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("unexpected"));
    }

    #[test]
    fn routine_contract_does_not_change_story_frontmatter_parsing() {
        let yaml = r#"
id: FEAT0001
title: Existing Story Contract
type: feat
status: backlog
scope: VDakm8eVW/VDcFd11nc
created_at: 2026-03-11T09:00:00
updated_at: 2026-03-11T09:30:00
started_at: 2026-03-11T10:00:00
"#;

        let frontmatter: StoryFrontmatter = serde_yaml::from_str(yaml).unwrap();
        let story = Story::new(frontmatter.clone(), ".keel/stories/FEAT0001/README.md");

        assert_eq!(frontmatter.id, "FEAT0001");
        assert_eq!(frontmatter.status, StoryState::Backlog);
        assert_eq!(story.id(), "FEAT0001");
        assert_eq!(story.status(), StoryState::Backlog);
        assert_eq!(story.epic(), Some("VDakm8eVW"));
        assert_eq!(story.voyage(), Some("VDcFd11nc"));
    }
}
