use std::fs;

use super::super::fixes::{TestIndex, extract_grep_command};
use super::super::types::*;
use crate::domain::model::Board;
use crate::infrastructure::verification::{Comparison, parse_verify_annotations};

/// Annotation coverage counts -- not a pass/fail check, just counts.
pub struct AnnotationCoverage {
    pub manual: usize,
    pub test: usize,
    pub grep: usize,
    pub other_executable: usize,
}

impl AnnotationCoverage {
    pub fn total(&self) -> usize {
        self.manual + self.test + self.grep + self.other_executable
    }

    pub fn executable(&self) -> usize {
        self.test + self.grep + self.other_executable
    }
}

/// Count verify annotations across all stories, categorised by type.
#[cfg(test)]
pub fn count_annotation_coverage(board: &Board) -> AnnotationCoverage {
    let (coverage, _) = analyze_annotations(board, &[]);
    coverage
}

/// Single-pass annotation analysis: counts coverage and detects migration candidates.
pub fn analyze_annotations(
    board: &Board,
    test_names: &[String],
) -> (AnnotationCoverage, Vec<Problem>) {
    let mut coverage = AnnotationCoverage {
        manual: 0,
        test: 0,
        grep: 0,
        other_executable: 0,
    };
    let mut problems = Vec::new();

    // Build inverted index once for O(words) matching instead of O(words * tests)
    let test_index = TestIndex::build(test_names.to_vec());

    for story in board.stories.values() {
        let content = match fs::read_to_string(&story.path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let annotations = parse_verify_annotations(&content);
        let mut migratable_test = 0usize;
        let mut migratable_grep = 0usize;

        for ann in &annotations {
            match ann.comparison {
                Comparison::Manual => {
                    coverage.manual += 1;
                    if !test_index.is_empty() && test_index.find_match(&ann.criterion).is_some() {
                        migratable_test += 1;
                    } else if extract_grep_command(&ann.criterion).is_some() {
                        migratable_grep += 1;
                    }
                }
                _ => {
                    if let Some(ref cmd) = ann.command {
                        if cmd.starts_with("test=") {
                            coverage.test += 1;
                        } else if cmd.contains("grep") {
                            coverage.grep += 1;
                        } else {
                            coverage.other_executable += 1;
                        }
                    } else {
                        coverage.other_executable += 1;
                    }
                }
            }
        }

        if migratable_test > 0 {
            problems.push(Problem {
                severity: Severity::Warning,
                path: story.path.clone(),
                message: format!(
                    "{}: {} manual annotation(s) could be migrated to test=",
                    story.id(),
                    migratable_test
                ),
                fix: Some(Fix::MigrateAnnotationToTest {
                    path: story.path.clone(),
                }),
                scope: story.scope().map(|s| s.to_string()),
                category: None,
                check_id: CheckId::Unknown,
            });
        }

        if migratable_grep > 0 {
            problems.push(Problem {
                severity: Severity::Warning,
                path: story.path.clone(),
                message: format!(
                    "{}: {} manual annotation(s) could be migrated to grep",
                    story.id(),
                    migratable_grep
                ),
                fix: Some(Fix::MigrateAnnotationToGrep {
                    path: story.path.clone(),
                }),
                scope: story.scope().map(|s| s.to_string()),
                category: None,
                check_id: CheckId::Unknown,
            });
        }
    }

    (coverage, problems)
}
