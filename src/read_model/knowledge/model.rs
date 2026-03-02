//! Knowledge model
//!
//! Represents knowledge extracted from stories, voyages, and ad-hoc files.

use chrono::{DateTime, Utc};
use std::fmt;
use std::path::PathBuf;

/// Source of an extracted knowledge
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KnowledgeSourceType {
    /// Knowledge from a completed story's ## Knowledge section
    Story,
    /// Knowledge from a voyage's KNOWLEDGE.md ## Synthesis section
    Voyage,
    /// Knowledge from an ad-hoc file in docs/knowledge/
    Adhoc,
}

impl fmt::Display for KnowledgeSourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KnowledgeSourceType::Story => write!(f, "story"),
            KnowledgeSourceType::Voyage => write!(f, "voyage"),
            KnowledgeSourceType::Adhoc => write!(f, "adhoc"),
        }
    }
}

/// A single unit of knowledge extracted from implementation or research
#[derive(Debug, Clone)]
pub struct Knowledge {
    /// Unique identifier (e.g., L001, ML001)
    pub id: String,
    /// Source file path
    pub source: PathBuf,
    /// Type of source (Story, Voyage, Adhoc)
    pub source_type: KnowledgeSourceType,
    /// Scope (epic/voyage) where it was discovered
    pub scope: Option<String>,
    /// Short descriptive title
    pub title: String,
    /// Thematic category (e.g., code, testing, process)
    pub category: String,
    /// Specific context where this applies
    pub context: String,
    /// The fundamental insight discovered
    pub insight: String,
    /// What should be done based on this insight
    pub suggested_action: String,
    /// Which files or patterns this insight applies to
    pub applies_to: String,
    /// Record of where this insight has already been applied
    pub applied: String,
    /// Timestamp of discovery (used for trend detection)
    pub observed_at: Option<DateTime<Utc>>,
    /// Numeric score 0.0-1.0 representing impact significance
    pub score: f64,
    /// Confidence 0.0-1.0 in the insight quality
    pub confidence: f64,
}

impl Knowledge {
    /// Check if the knowledge has been applied
    pub fn is_applied(&self) -> bool {
        !self.applied.is_empty()
    }

    /// Check if the knowledge is pending application
    pub fn is_pending(&self) -> bool {
        self.applied.is_empty()
    }

    /// Convert knowledge to a reflection signal for pattern detection
    pub fn to_signal(&self) -> Option<super::navigator::ReflectionSignal> {
        let observed_at = self.observed_at?;

        Some(super::navigator::ReflectionSignal {
            context_id: self.scope.clone(),
            focus_area: Some(self.category.clone()),
            score: self.score,
            confidence: self.confidence,
            observed_at,
            evidence_id: self.id.clone(),
        })
    }
}

/// Aggregated summary of knowledge across the project
#[derive(Debug, Default)]
pub struct KnowledgeSummary {
    pub total_count: usize,
    pub pending_count: usize,
    pub applied_count: usize,
    pub by_category: std::collections::HashMap<String, usize>,
}

impl KnowledgeSummary {
    /// Calculate summary from a list of knowledge units
    pub fn from_knowledge(knowledge_list: &[Knowledge]) -> Self {
        let mut summary = KnowledgeSummary {
            total_count: knowledge_list.len(),
            ..Default::default()
        };

        for k in knowledge_list {
            if k.is_applied() {
                summary.applied_count += 1;
            } else {
                summary.pending_count += 1;
            }

            *summary.by_category.entry(k.category.clone()).or_insert(0) += 1;
        }

        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn knowledge_is_pending_when_field_is_empty() {
        let k = Knowledge {
            id: "L001".to_string(),
            source: Path::new("test.md").to_path_buf(),
            source_type: KnowledgeSourceType::Story,
            scope: None,
            title: "Test".to_string(),
            category: "code".to_string(),
            context: String::new(),
            insight: "Insight".to_string(),
            suggested_action: String::new(),
            applies_to: String::new(),
            applied: String::new(),
            observed_at: None,
            score: 0.5,
            confidence: 0.8,
        };
        assert!(k.is_pending());
        assert!(!k.is_applied());
    }

    #[test]
    fn knowledge_is_applied_when_field_has_content() {
        let k = Knowledge {
            id: "L001".to_string(),
            source: Path::new("test.md").to_path_buf(),
            source_type: KnowledgeSourceType::Story,
            scope: None,
            title: "Test".to_string(),
            category: "code".to_string(),
            context: String::new(),
            insight: "Insight".to_string(),
            suggested_action: String::new(),
            applies_to: String::new(),
            applied: "Applied in CLAUDE.md".to_string(),
            observed_at: None,
            score: 0.5,
            confidence: 0.8,
        };
        assert!(!k.is_pending());
        assert!(k.is_applied());
    }

    #[test]
    fn summary_handles_empty_knowledge() {
        let summary = KnowledgeSummary::from_knowledge(&[]);
        assert_eq!(summary.total_count, 0);
    }

    #[test]
    fn summary_calculates_correctly() {
        let list = vec![
            Knowledge {
                id: "L001".to_string(),
                source: Path::new("a.md").to_path_buf(),
                source_type: KnowledgeSourceType::Story,
                scope: None,
                title: "A".to_string(),
                category: "code".to_string(),
                context: String::new(),
                insight: "I1".to_string(),
                suggested_action: String::new(),
                applies_to: String::new(),
                applied: "done".to_string(),
                observed_at: None,
                score: 0.5,
                confidence: 0.8,
            },
            Knowledge {
                id: "L002".to_string(),
                source: Path::new("b.md").to_path_buf(),
                source_type: KnowledgeSourceType::Story,
                scope: None,
                title: "B".to_string(),
                category: "code".to_string(),
                context: String::new(),
                insight: "I2".to_string(),
                suggested_action: String::new(),
                applies_to: String::new(),
                applied: String::new(),
                observed_at: None,
                score: 0.5,
                confidence: 0.8,
            },
            Knowledge {
                id: "L003".to_string(),
                source: Path::new("c.md").to_path_buf(),
                source_type: KnowledgeSourceType::Story,
                scope: None,
                title: "C".to_string(),
                category: "process".to_string(),
                context: String::new(),
                insight: "I3".to_string(),
                suggested_action: String::new(),
                applies_to: String::new(),
                applied: String::new(),
                observed_at: None,
                score: 0.5,
                confidence: 0.8,
            },
        ];

        let summary = KnowledgeSummary::from_knowledge(&list);
        assert_eq!(summary.total_count, 3);
        assert_eq!(summary.applied_count, 1);
        assert_eq!(summary.pending_count, 2);
        assert_eq!(summary.by_category.get("code"), Some(&2));
        assert_eq!(summary.by_category.get("process"), Some(&1));
    }

    #[test]
    fn source_type_display() {
        assert_eq!(KnowledgeSourceType::Story.to_string(), "story");
        assert_eq!(KnowledgeSourceType::Voyage.to_string(), "voyage");
        assert_eq!(KnowledgeSourceType::Adhoc.to_string(), "adhoc");
    }

    #[test]
    fn knowledge_is_pending_when_field_is_whitespace() {
        let k = Knowledge {
            id: "L001".to_string(),
            source: Path::new("test.md").to_path_buf(),
            source_type: KnowledgeSourceType::Story,
            scope: None,
            title: "Test".to_string(),
            category: "code".to_string(),
            context: String::new(),
            insight: "Insight".to_string(),
            suggested_action: String::new(),
            applies_to: String::new(),
            applied: "   ".to_string(),
            observed_at: None,
            score: 0.5,
            confidence: 0.8,
        };
        assert!(k.is_applied());
    }
}
