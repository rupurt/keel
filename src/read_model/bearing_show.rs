//! Canonical bearing-show projection and markdown extraction helpers.

use crate::read_model::planning_show::extract_section;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BearingShowProjection {
    pub brief: BearingBriefSummary,
    pub survey: Option<BearingSurveySummary>,
    pub assessment: Option<BearingAssessmentSummary>,
    pub frontmatter_datetimes: Vec<FrontmatterDatetimeField>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BearingBriefSummary {
    pub hypothesis: Option<String>,
    pub problem_space: Option<String>,
    pub checked_success_criteria: usize,
    pub total_success_criteria: usize,
    pub unchecked_success_criteria: Vec<String>,
    pub open_questions: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BearingSurveySummary {
    pub feasibility: Option<String>,
    pub key_findings: Vec<String>,
    pub unknowns: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BearingAssessmentSummary {
    pub recommendation: Option<String>,
    pub opportunity_cost: Option<String>,
    pub dependencies: Vec<String>,
    pub alternatives: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FrontmatterDatetimeField {
    pub key: String,
    pub label: String,
    pub value: String,
}

pub fn build_bearing_show_projection(
    readme_content: &str,
    brief_content: &str,
    survey_content: Option<&str>,
    assessment_content: Option<&str>,
) -> BearingShowProjection {
    let success_criteria =
        parse_checkbox_items(&section_text(brief_content, "## Success Criteria"));

    let unchecked_success_criteria = success_criteria
        .iter()
        .filter(|(checked, _)| !checked)
        .map(|(_, value)| value.clone())
        .collect();

    let brief = BearingBriefSummary {
        hypothesis: first_authored_non_list_text(&section_text(brief_content, "## Hypothesis")),
        problem_space: first_authored_non_list_text(&section_text(
            brief_content,
            "## Problem Space",
        )),
        checked_success_criteria: success_criteria
            .iter()
            .filter(|(checked, _)| *checked)
            .count(),
        total_success_criteria: success_criteria.len(),
        unchecked_success_criteria,
        open_questions: parse_markdown_list_items(&section_text(
            brief_content,
            "## Open Questions",
        )),
    };

    let survey = survey_content.map(|content| BearingSurveySummary {
        feasibility: first_authored_non_list_text(&section_text(content, "### Feasibility")),
        key_findings: parse_markdown_list_items(&section_text(content, "## Key Findings")),
        unknowns: parse_markdown_list_items(&section_text(content, "## Unknowns")),
    });

    let assessment = assessment_content.map(|content| {
        let recommendation_section = section_text(content, "## Recommendation");
        BearingAssessmentSummary {
            recommendation: extract_checked_recommendation(&recommendation_section),
            opportunity_cost: first_authored_non_list_text(&section_text(
                content,
                "### Opportunity Cost",
            )),
            dependencies: parse_markdown_list_items(&section_text(content, "### Dependencies")),
            alternatives: parse_markdown_list_items(&section_text(
                content,
                "### Alternatives Considered",
            )),
        }
    });

    BearingShowProjection {
        brief,
        survey,
        assessment,
        frontmatter_datetimes: extract_frontmatter_datetime_fields(readme_content),
    }
}

fn section_text(content: &str, heading: &str) -> String {
    extract_section(content, heading).unwrap_or_default()
}

fn first_authored_non_list_text(section: &str) -> Option<String> {
    section
        .lines()
        .map(str::trim)
        .find(|line| {
            !line.is_empty()
                && !line.starts_with("<!--")
                && !line.starts_with('-')
                && !line.starts_with('|')
                && parse_ordered_list_item(line).is_none()
        })
        .map(ToOwned::to_owned)
}

fn parse_markdown_list_items(section: &str) -> Vec<String> {
    let mut items = Vec::new();
    for line in section.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("<!--") {
            continue;
        }

        if let Some(item) = trimmed
            .strip_prefix("- [ ] ")
            .map(str::trim)
            .or_else(|| trimmed.strip_prefix("- [x] ").map(str::trim))
            .or_else(|| trimmed.strip_prefix("- [X] ").map(str::trim))
        {
            if !item.is_empty() {
                items.push(item.to_string());
            }
            continue;
        }

        if let Some(item) = trimmed.strip_prefix("- ").map(str::trim) {
            if !item.is_empty() {
                items.push(item.to_string());
            }
            continue;
        }

        if let Some(item) = parse_ordered_list_item(trimmed) {
            if !item.is_empty() {
                items.push(item.to_string());
            }
            continue;
        }
    }
    items
}

fn parse_ordered_list_item(line: &str) -> Option<&str> {
    let mut digits = 0usize;
    for byte in line.as_bytes() {
        if byte.is_ascii_digit() {
            digits += 1;
        } else {
            break;
        }
    }
    if digits == 0 || !line[digits..].starts_with(". ") {
        return None;
    }

    Some(line[digits + 2..].trim())
}

fn parse_checkbox_items(section: &str) -> Vec<(bool, String)> {
    let mut items = Vec::new();
    for line in section.lines() {
        let trimmed = line.trim();
        if let Some(item) = trimmed.strip_prefix("- [ ] ").map(str::trim) {
            if !item.is_empty() {
                items.push((false, item.to_string()));
            }
            continue;
        }
        if let Some(item) = trimmed
            .strip_prefix("- [x] ")
            .map(str::trim)
            .or_else(|| trimmed.strip_prefix("- [X] ").map(str::trim))
            .filter(|item| !item.is_empty())
        {
            items.push((true, item.to_string()));
        }
    }
    items
}

fn extract_checked_recommendation(section: &str) -> Option<String> {
    for line in section.lines() {
        let trimmed = line.trim();
        if let Some(value) = trimmed
            .strip_prefix("[x] ")
            .or_else(|| trimmed.strip_prefix("[X] "))
            .or_else(|| trimmed.strip_prefix("- [x] "))
            .or_else(|| trimmed.strip_prefix("- [X] "))
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            return Some(value.to_string());
        }
    }
    None
}

fn extract_frontmatter_datetime_fields(content: &str) -> Vec<FrontmatterDatetimeField> {
    let mut fields = Vec::new();
    let mut fence_count = 0usize;
    let mut in_frontmatter = false;

    for line in content.lines() {
        if line.trim() == "---" {
            fence_count += 1;
            in_frontmatter = fence_count == 1;
            if fence_count >= 2 {
                break;
            }
            continue;
        }
        if !in_frontmatter {
            continue;
        }

        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            let value = value.trim();
            if key.ends_with("_at") && !value.is_empty() {
                fields.push(FrontmatterDatetimeField {
                    key: key.to_string(),
                    label: datetime_field_label(key),
                    value: value.to_string(),
                });
            }
        }
    }

    fields.sort_by(|a, b| a.key.cmp(&b.key));
    fields
}

fn datetime_field_label(key: &str) -> String {
    let base = key.strip_suffix("_at").unwrap_or(key);
    base.split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            let Some(first) = chars.next() else {
                return String::new();
            };
            format!("{}{}", first.to_ascii_uppercase(), chars.as_str())
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_markdown_list_items_supports_bullets_and_ordered_items() {
        let section = r#"
- first
- second
1. third
2. fourth
"#;
        assert_eq!(
            parse_markdown_list_items(section),
            vec![
                "first".to_string(),
                "second".to_string(),
                "third".to_string(),
                "fourth".to_string()
            ]
        );
    }

    #[test]
    fn extract_checked_recommendation_returns_selected_option() {
        let section = r#"
[ ] Proceed → convert to epic
[x] Park → revisit later
[ ] Decline → document learnings
"#;
        assert_eq!(
            extract_checked_recommendation(section).as_deref(),
            Some("Park → revisit later")
        );
    }

    #[test]
    fn extract_frontmatter_datetime_fields_collects_all_at_keys() {
        let content = r#"---
id: test
created_at: 2026-03-05T10:00:00
laid_at: 2026-03-06T10:00:00
surveyed_at: 2026-03-07T10:00:00
---

# Test
"#;
        assert_eq!(
            extract_frontmatter_datetime_fields(content),
            vec![
                FrontmatterDatetimeField {
                    key: "created_at".to_string(),
                    label: "Created".to_string(),
                    value: "2026-03-05T10:00:00".to_string(),
                },
                FrontmatterDatetimeField {
                    key: "laid_at".to_string(),
                    label: "Laid".to_string(),
                    value: "2026-03-06T10:00:00".to_string(),
                },
                FrontmatterDatetimeField {
                    key: "surveyed_at".to_string(),
                    label: "Surveyed".to_string(),
                    value: "2026-03-07T10:00:00".to_string(),
                },
            ]
        );
    }

    #[test]
    fn build_bearing_show_projection_extracts_sections() {
        let readme = r#"---
id: B1
created_at: 2026-03-05T10:00:00
reviewed_at: 2026-03-06T12:00:00
---
"#;
        let brief = r#"
## Hypothesis
**Users need speed**

## Problem Space
*Fast feedback loops* reduce coordination drag.

## Success Criteria
- [x] Criterion one
- [ ] Criterion two

## Open Questions
- What is the team capacity?
"#;
        let survey = r#"
### Feasibility
Looks practical.

## Key Findings
1. Existing tools are close.

## Unknowns
- Long-tail migration effort
"#;
        let assessment = r#"
## Analysis
### Opportunity Cost
Delayed roadmap item.

### Dependencies
- Team bandwidth

### Alternatives Considered
- Delay and observe

## Recommendation
[x] Proceed
[ ] Park
"#;

        let projection =
            build_bearing_show_projection(readme, brief, Some(survey), Some(assessment));

        assert_eq!(
            projection.brief.hypothesis.as_deref(),
            Some("**Users need speed**")
        );
        assert_eq!(projection.brief.checked_success_criteria, 1);
        assert_eq!(projection.brief.total_success_criteria, 2);
        assert_eq!(
            projection.brief.unchecked_success_criteria,
            vec!["Criterion two"]
        );
        assert_eq!(
            projection.brief.open_questions,
            vec!["What is the team capacity?"]
        );

        let survey = projection.survey.unwrap();
        assert_eq!(survey.feasibility.as_deref(), Some("Looks practical."));
        assert_eq!(survey.key_findings, vec!["Existing tools are close."]);
        assert_eq!(survey.unknowns, vec!["Long-tail migration effort"]);

        let assessment = projection.assessment.unwrap();
        assert_eq!(assessment.recommendation.as_deref(), Some("Proceed"));
        assert_eq!(
            assessment.opportunity_cost.as_deref(),
            Some("Delayed roadmap item.")
        );
        assert_eq!(assessment.dependencies, vec!["Team bandwidth"]);
        assert_eq!(assessment.alternatives, vec!["Delay and observe"]);
        assert_eq!(projection.frontmatter_datetimes.len(), 2);
    }
}
