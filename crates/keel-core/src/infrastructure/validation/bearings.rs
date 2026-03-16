//! Bearing-specific validation logic

use std::path::Path;

use crate::domain::model::{Bearing, Board};
use crate::infrastructure::markdown_sections::extract_section;
use crate::infrastructure::validation::structural::first_unfilled_placeholder_pattern;
use crate::infrastructure::validation::types::{CheckId, Problem, Severity};
use std::fs;

/// Required sections for a bearing's BRIEF.md
pub const BEARING_REQUIRED_SECTIONS: &[&str] = &[
    "## Hypothesis",
    "## Problem Space",
    "## Success Criteria",
    "## Open Questions",
];

const HYPOTHESIS_SCAFFOLD_MARKERS: &[&str] = &["What we believe and why it might matter."];
const PROBLEM_SPACE_SCAFFOLD_MARKERS: &[&str] =
    &["What problem or opportunity are we investigating?"];
const SUCCESS_CRITERIA_SCAFFOLD_MARKERS: &[&str] = &[
    "How will we know if this research was valuable?",
    "Criterion 1",
    "Criterion 2",
];
const OPEN_QUESTION_SCAFFOLD_MARKERS: &[&str] = &["Question we need to answer"];

/// Check bearing content sections
/// Validates that BRIEF.md, EVIDENCE.md, and ASSESSMENT.md contain required markdown sections
pub fn check_bearing_content_sections(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for bearing in board.bearings.values() {
        problems.extend(check_bearing_content_sections_for_bearing(bearing));

        let evidence_path = bearing.path.parent().unwrap().join("EVIDENCE.md");
        if let Ok(content) = fs::read_to_string(&evidence_path) {
            // Sources validation (legacy)
            for message in
                crate::infrastructure::bearing_evidence::validate_evidence_document(&content)
            {
                problems.push(Problem {
                    severity: Severity::Error,
                    path: evidence_path.clone(),
                    message: format!(
                        "bearing '{}' evidence contract error: {}",
                        bearing.id(),
                        message
                    ),
                    fix: None,
                    scope: None,
                    category: None,
                    check_id: CheckId::Unknown,
                });
            }

            // New strict contract validation for EVIDENCE.md
            for heading in ["## Feasibility", "## Key Findings", "## Unknowns"] {
                if extract_section(&content, heading).is_none() {
                    problems.push(Problem {
                        severity: Severity::Error,
                        path: evidence_path.clone(),
                        message: format!(
                            "bearing '{}' is missing required EVIDENCE.md section: {}",
                            bearing.id(),
                            heading
                        ),
                        fix: None,
                        scope: None,
                        category: None,
                        check_id: CheckId::Unknown,
                    });
                }
            }
        }

        let assessment_path = bearing.path.parent().unwrap().join("ASSESSMENT.md");
        if let Ok(content) = fs::read_to_string(&assessment_path) {
            for heading in [
                "## Scoring Factors",
                "## Findings",
                "## Opportunity Cost",
                "## Dependencies",
                "## Alternatives Considered",
                "## Recommendation",
            ] {
                if extract_section(&content, heading).is_none() {
                    problems.push(Problem {
                        severity: Severity::Error,
                        path: assessment_path.clone(),
                        message: format!(
                            "bearing '{}' is missing required ASSESSMENT.md section: {}",
                            bearing.id(),
                            heading
                        ),
                        fix: None,
                        scope: None,
                        category: None,
                        check_id: CheckId::Unknown,
                    });
                }
            }
        }
    }

    problems
}

/// Validate bearing dependency references.
///
/// Checks that every ID in `depends_on` references an existing bearing,
/// flags self-references, and detects cycles via DFS.
pub fn check_bearing_dependencies(board: &Board) -> Vec<Problem> {
    use std::collections::{HashMap, HashSet};

    let mut problems = Vec::new();
    let bearing_ids: HashSet<&str> = board.bearings.keys().map(|k| k.as_str()).collect();

    // Build adjacency list and check dangling refs + self-refs
    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
    for bearing in board.bearings.values() {
        let deps = match &bearing.frontmatter.depends_on {
            Some(deps) if !deps.is_empty() => deps,
            _ => continue,
        };

        let mut edges = Vec::new();
        for dep in deps {
            if dep == bearing.id() {
                problems.push(Problem {
                    severity: Severity::Error,
                    path: bearing.path.clone(),
                    message: format!("bearing '{}' references itself in depends_on", bearing.id()),
                    fix: None,
                    scope: None,
                    category: None,
                    check_id: CheckId::BearingDependencyCycle,
                });
            } else if !bearing_ids.contains(dep.as_str()) {
                problems.push(Problem {
                    severity: Severity::Error,
                    path: bearing.path.clone(),
                    message: format!(
                        "bearing '{}' depends_on '{}' which does not exist",
                        bearing.id(),
                        dep
                    ),
                    fix: None,
                    scope: None,
                    category: None,
                    check_id: CheckId::BearingDanglingDependency,
                });
            } else {
                edges.push(dep.as_str());
            }
        }
        adj.insert(bearing.id(), edges);
    }

    // DFS cycle detection with coloring: White(unvisited), Gray(in-stack), Black(done)
    #[derive(Clone, Copy, PartialEq)]
    enum Color {
        White,
        Gray,
        Black,
    }

    let mut color: HashMap<&str, Color> =
        bearing_ids.iter().map(|&id| (id, Color::White)).collect();
    let mut cycle_members: HashSet<&str> = HashSet::new();

    fn dfs<'a>(
        node: &'a str,
        adj: &HashMap<&'a str, Vec<&'a str>>,
        color: &mut HashMap<&'a str, Color>,
        cycle_members: &mut HashSet<&'a str>,
    ) {
        color.insert(node, Color::Gray);
        if let Some(neighbors) = adj.get(node) {
            for &next in neighbors {
                match color.get(next) {
                    Some(Color::Gray) => {
                        cycle_members.insert(node);
                        cycle_members.insert(next);
                    }
                    Some(Color::White) => {
                        dfs(next, adj, color, cycle_members);
                        if cycle_members.contains(next) {
                            cycle_members.insert(node);
                        }
                    }
                    _ => {}
                }
            }
        }
        color.insert(node, Color::Black);
    }

    for &id in &bearing_ids {
        if color[id] == Color::White {
            dfs(id, &adj, &mut color, &mut cycle_members);
        }
    }

    // Report cycles (deterministic order)
    let mut cycle_list: Vec<&str> = cycle_members.into_iter().collect();
    cycle_list.sort();
    for id in cycle_list {
        if let Some(bearing) = board.bearings.get(id) {
            problems.push(Problem {
                severity: Severity::Error,
                path: bearing.path.clone(),
                message: format!(
                    "bearing '{}' is part of a dependency cycle in depends_on",
                    id
                ),
                fix: None,
                scope: None,
                category: None,
                check_id: CheckId::BearingDependencyCycle,
            });
        }
    }

    problems
}

pub fn check_bearing_content_sections_for_bearing(bearing: &Bearing) -> Vec<Problem> {
    let brief_path = bearing.path.parent().unwrap().join("BRIEF.md");
    check_bearing_content_sections_for_path(bearing.id(), &brief_path)
}

pub fn check_bearing_content_sections_for_path(
    bearing_id: &str,
    brief_path: &Path,
) -> Vec<Problem> {
    let content = match fs::read_to_string(brief_path) {
        Ok(content) => content,
        Err(_) => return Vec::new(), // File read errors handled elsewhere
    };

    let mut problems = Vec::new();
    for requirement in brief_requirements() {
        let Some(section) = extract_section(&content, requirement.heading) else {
            problems.push(Problem {
                severity: Severity::Error,
                path: brief_path.to_path_buf(),
                message: format!(
                    "bearing '{}' is missing required BRIEF.md section: {}",
                    bearing_id, requirement.label
                ),
                fix: None,
                scope: None,
                category: None,
                check_id: CheckId::Unknown,
            });
            continue;
        };

        if brief_section_uses_scaffold(&section, requirement.scaffold_markers) {
            problems.push(Problem {
                severity: Severity::Error,
                path: brief_path.to_path_buf(),
                message: format!(
                    "bearing '{}' must replace the BRIEF.md {} scaffold with authored content",
                    bearing_id, requirement.label
                ),
                fix: None,
                scope: None,
                category: None,
                check_id: CheckId::Unknown,
            });
        }
    }

    problems
}

struct BriefRequirement {
    heading: &'static str,
    label: &'static str,
    scaffold_markers: &'static [&'static str],
}

fn brief_requirements() -> [BriefRequirement; 4] {
    [
        BriefRequirement {
            heading: "## Hypothesis",
            label: "Hypothesis",
            scaffold_markers: HYPOTHESIS_SCAFFOLD_MARKERS,
        },
        BriefRequirement {
            heading: "## Problem Space",
            label: "Problem Space",
            scaffold_markers: PROBLEM_SPACE_SCAFFOLD_MARKERS,
        },
        BriefRequirement {
            heading: "## Success Criteria",
            label: "Success Criteria",
            scaffold_markers: SUCCESS_CRITERIA_SCAFFOLD_MARKERS,
        },
        BriefRequirement {
            heading: "## Open Questions",
            label: "Open Questions",
            scaffold_markers: OPEN_QUESTION_SCAFFOLD_MARKERS,
        },
    ]
}

fn brief_section_uses_scaffold(section: &str, scaffold_markers: &[&str]) -> bool {
    first_unfilled_placeholder_pattern(section).is_some()
        || scaffold_markers
            .iter()
            .any(|marker| section.contains(marker))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::loader::load_board;
    use crate::test_helpers::{TestBearing, TestBoardBuilder};

    #[test]
    fn doctor_flags_scaffold_hypothesis_and_problem_space() {
        let temp = TestBoardBuilder::new()
            .bearing(TestBearing::new("BRG-01"))
            .build();
        fs::write(
            temp.path().join("bearings/BRG-01/BRIEF.md"),
            r#"# Test Bearing — Brief

## Hypothesis

What we believe and why it might matter.

## Problem Space

What problem or opportunity are we investigating?

## Success Criteria

How will we know if this research was valuable?

- [ ] Criterion 1
- [ ] Criterion 2

## Open Questions

- Question we need to answer
"#,
        )
        .unwrap();

        let board = load_board(temp.path()).unwrap();
        let problems = check_bearing_content_sections(&board);

        assert!(
            problems
                .iter()
                .any(|problem| problem.message.contains("BRIEF.md Hypothesis scaffold"))
        );
        assert!(
            problems
                .iter()
                .any(|problem| problem.message.contains("BRIEF.md Problem Space scaffold"))
        );
    }

    #[test]
    fn doctor_flags_missing_evidence_sections() {
        let temp = TestBoardBuilder::new()
            .bearing(TestBearing::new("BRG-01"))
            .build();

        // Write EVIDENCE.md missing all required sections
        fs::write(
            temp.path().join("bearings/BRG-01/EVIDENCE.md"),
            "---\nid: BRG-01\n---\n\n# BRG-01 — Evidence\n\n## Sources\n\nNo sources yet.\n",
        )
        .unwrap();

        let board = load_board(temp.path()).unwrap();
        let problems = check_bearing_content_sections(&board);

        for heading in ["## Feasibility", "## Key Findings", "## Unknowns"] {
            assert!(
                problems
                    .iter()
                    .any(|p| p.message.contains("EVIDENCE.md section")
                        && p.message.contains(heading)),
                "expected a problem for missing {}",
                heading
            );
        }
    }

    #[test]
    fn doctor_flags_missing_assessment_sections() {
        let temp = TestBoardBuilder::new()
            .bearing(TestBearing::new("BRG-01"))
            .build();

        // Write ASSESSMENT.md missing all required sections
        fs::write(
            temp.path().join("bearings/BRG-01/ASSESSMENT.md"),
            "# BRG-01 — Assessment\n\nSome notes.\n",
        )
        .unwrap();

        let board = load_board(temp.path()).unwrap();
        let problems = check_bearing_content_sections(&board);

        for heading in [
            "## Scoring Factors",
            "## Findings",
            "## Opportunity Cost",
            "## Dependencies",
            "## Alternatives Considered",
            "## Recommendation",
        ] {
            assert!(
                problems
                    .iter()
                    .any(|p| p.message.contains("ASSESSMENT.md section")
                        && p.message.contains(heading)),
                "expected a problem for missing {}",
                heading
            );
        }
    }

    #[test]
    fn doctor_accepts_complete_evidence_and_assessment() {
        let temp = TestBoardBuilder::new()
            .bearing(
                TestBearing::new("BRG-01")
                    .has_evidence(true)
                    .has_assessment(true),
            )
            .build();

        // Fixture builder now writes complete EVIDENCE.md and ASSESSMENT.md
        let board = load_board(temp.path()).unwrap();
        let problems = check_bearing_content_sections(&board);

        let evidence_problems: Vec<_> = problems
            .iter()
            .filter(|p| p.message.contains("EVIDENCE.md section"))
            .collect();
        assert!(
            evidence_problems.is_empty(),
            "expected no EVIDENCE.md section errors, got: {:?}",
            evidence_problems
                .iter()
                .map(|p| &p.message)
                .collect::<Vec<_>>()
        );

        let assessment_problems: Vec<_> = problems
            .iter()
            .filter(|p| p.message.contains("ASSESSMENT.md section"))
            .collect();
        assert!(
            assessment_problems.is_empty(),
            "expected no ASSESSMENT.md section errors, got: {:?}",
            assessment_problems
                .iter()
                .map(|p| &p.message)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn doctor_accepts_authored_bearing_brief_sections() {
        let temp = TestBoardBuilder::new()
            .bearing(TestBearing::new("BRG-01"))
            .build();

        let brief_path = temp.path().join("bearings/BRG-01/BRIEF.md");
        fs::write(
            &brief_path,
            r#"# Test Bearing — Brief

## Hypothesis

Operators will make faster planning decisions when the research framing is explicit.

## Problem Space

The current bearing loop makes it easy to advance without a concrete hypothesis or problem statement.

## Success Criteria

- [ ] Research framing is authored before the bearing advances.

## Open Questions

- Should lay also surface the same recovery guidance?
"#,
        )
        .unwrap();

        let board = load_board(temp.path()).unwrap();
        let problems = check_bearing_content_sections(&board);
        assert!(problems.is_empty());
    }

    #[test]
    fn doctor_flags_dangling_depends_on() {
        let temp = TestBoardBuilder::new()
            .bearing(TestBearing::new("BRG-01").depends_on(vec!["NONEXISTENT"]))
            .build();

        let board = load_board(temp.path()).unwrap();
        let problems = check_bearing_dependencies(&board);

        assert_eq!(problems.len(), 1);
        assert!(problems[0].message.contains("does not exist"));
        assert_eq!(problems[0].check_id, CheckId::BearingDanglingDependency);
    }

    #[test]
    fn doctor_flags_self_reference_in_depends_on() {
        let temp = TestBoardBuilder::new()
            .bearing(TestBearing::new("BRG-01").depends_on(vec!["BRG-01"]))
            .build();

        let board = load_board(temp.path()).unwrap();
        let problems = check_bearing_dependencies(&board);

        assert!(
            problems
                .iter()
                .any(|p| p.message.contains("references itself")),
        );
        assert!(
            problems
                .iter()
                .any(|p| p.check_id == CheckId::BearingDependencyCycle),
        );
    }

    #[test]
    fn doctor_flags_cyclic_depends_on() {
        let temp = TestBoardBuilder::new()
            .bearing(TestBearing::new("BRG-01").depends_on(vec!["BRG-02"]))
            .bearing(TestBearing::new("BRG-02").depends_on(vec!["BRG-01"]))
            .build();

        let board = load_board(temp.path()).unwrap();
        let problems = check_bearing_dependencies(&board);

        let cycle_problems: Vec<_> = problems
            .iter()
            .filter(|p| p.message.contains("dependency cycle"))
            .collect();
        assert_eq!(cycle_problems.len(), 2, "both bearings should be flagged");
    }

    #[test]
    fn doctor_accepts_valid_depends_on() {
        let temp = TestBoardBuilder::new()
            .bearing(TestBearing::new("BRG-01"))
            .bearing(TestBearing::new("BRG-02").depends_on(vec!["BRG-01"]))
            .build();

        let board = load_board(temp.path()).unwrap();
        let problems = check_bearing_dependencies(&board);

        assert!(problems.is_empty());
    }

    #[test]
    fn dependency_validation_scales_linearly() {
        let count = 20;
        let mut builder = TestBoardBuilder::new();
        for i in 0..count {
            let id = format!("1w5H2B{:03}", i);
            let dep = if i > 0 {
                vec![format!("1w5H2B{:03}", i - 1)]
            } else {
                vec![]
            };
            let bearing = if dep.is_empty() {
                TestBearing::new(&id)
            } else {
                TestBearing::new(&id).depends_on(dep.iter().map(|s| s.as_str()).collect())
            };
            builder = builder.bearing(bearing);
        }
        let temp = builder.build();

        let board = load_board(temp.path()).unwrap();
        let problems = check_bearing_dependencies(&board);
        // Linear chain with no cycles should produce zero problems
        assert!(problems.is_empty());
    }
}
