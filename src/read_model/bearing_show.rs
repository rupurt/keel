//! Canonical bearing-show projection and markdown extraction helpers.

use std::collections::BTreeSet;

use crate::infrastructure::bearing_evidence::parse_evidence_records;
use crate::infrastructure::markdown_sections::{
    SectionExcerpt, extract_section, extract_section_excerpt, parse_markdown_list_items,
};
use crate::infrastructure::scoring::parse_assessment_document;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BearingShowProjection {
    pub brief: BearingBriefSummary,
    pub evidence: Option<BearingEvidenceSummary>,
    pub assessment: Option<BearingAssessmentSummary>,
    pub frontmatter_datetimes: Vec<FrontmatterDatetimeField>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BearingBriefSummary {
    pub hypothesis: Option<SectionExcerpt>,
    pub problem_space: Option<SectionExcerpt>,
    pub checked_success_criteria: usize,
    pub total_success_criteria: usize,
    pub unchecked_success_criteria: Vec<String>,
    pub open_questions: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BearingEvidenceSummary {
    pub feasibility: Option<SectionExcerpt>,
    pub key_findings: Vec<String>,
    pub unknowns: Vec<String>,
    pub sources: Vec<BearingEvidenceSourceSummary>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BearingEvidenceSourceSummary {
    pub id: String,
    pub class: String,
    pub provenance: String,
    pub authority: String,
    pub freshness: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BearingAssessmentSummary {
    pub recommendation: Option<String>,
    pub cited_sources: Vec<String>,
    pub findings: Vec<String>,
    pub opportunity_cost: Option<SectionExcerpt>,
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
    evidence_content: Option<&str>,
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
        hypothesis: extract_section_excerpt(&section_text(brief_content, "## Hypothesis")),
        problem_space: extract_section_excerpt(&section_text(brief_content, "## Problem Space")),
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

    let evidence = evidence_content.map(|content| BearingEvidenceSummary {
        feasibility: extract_section_excerpt(&section_text(content, "### Feasibility")),
        key_findings: parse_markdown_list_items(&section_text(content, "## Key Findings")),
        unknowns: parse_markdown_list_items(&section_text(content, "## Unknowns")),
        sources: parse_evidence_records(content)
            .unwrap_or_default()
            .into_iter()
            .map(|record| BearingEvidenceSourceSummary {
                id: record.id,
                class: record.class.to_string(),
                provenance: record.provenance,
                authority: record.authority.to_string(),
                freshness: record.freshness.to_string(),
            })
            .collect(),
    });

    let assessment = assessment_content.map(|content| {
        let document = parse_assessment_document(content);
        let recommendation_section = section_text(content, "## Recommendation");
        BearingAssessmentSummary {
            recommendation: extract_checked_recommendation(&recommendation_section),
            cited_sources: collect_cited_sources(&document),
            findings: parse_markdown_list_items(&section_text(content, "### Findings")),
            opportunity_cost: extract_section_excerpt(&section_text(
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
        evidence,
        assessment,
        frontmatter_datetimes: extract_frontmatter_datetime_fields(readme_content),
    }
}

fn section_text(content: &str, heading: &str) -> String {
    extract_section(content, heading).unwrap_or_default()
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

fn collect_cited_sources(
    document: &crate::infrastructure::scoring::AssessmentDocument,
) -> Vec<String> {
    let mut cited_sources = BTreeSet::new();

    for item in &document.findings {
        cited_sources.extend(item.citations.iter().cloned());
    }
    if let Some(opportunity_cost) = &document.opportunity_cost {
        cited_sources.extend(opportunity_cost.citations.iter().cloned());
    }
    for item in &document.dependencies {
        cited_sources.extend(item.citations.iter().cloned());
    }
    for item in &document.alternatives {
        cited_sources.extend(item.citations.iter().cloned());
    }
    if let Some(recommendation) = &document.recommendation {
        cited_sources.extend(recommendation.citations.iter().cloned());
    }

    cited_sources.into_iter().collect()
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
        let evidence = r#"
## Sources
| ID | Class | Provenance | Location | Observed / Published | Retrieved | Authority | Freshness | Notes |
|----|-------|------------|----------|----------------------|-----------|-----------|-----------|-------|
| SRC-01 | academic | manual:prior-art-review | https://example.test/paper | 2026-03-01 | 2026-03-02 | high | medium | Prior art supports the direction. |
| SRC-02 | web | manual:official-doc | https://example.test/docs | 2026-03-02 | 2026-03-03 | medium | high | Official docs confirm the implementation path. |

### Feasibility
Looks practical.

## Key Findings
1. Existing tools are close.

## Unknowns
- Long-tail migration effort
"#;
        let assessment = r#"
## Analysis
### Findings
- Finding supported by evidence [SRC-01]

### Opportunity Cost
Delayed roadmap item [SRC-02].

### Dependencies
- Team bandwidth [SRC-02]

### Alternatives Considered
- Delay and observe

## Recommendation
[x] Proceed [SRC-01][SRC-02]
[ ] Park
"#;

        let projection =
            build_bearing_show_projection(readme, brief, Some(evidence), Some(assessment));

        assert_eq!(
            projection
                .brief
                .hypothesis
                .as_ref()
                .map(|excerpt| excerpt.paragraphs.clone()),
            Some(vec!["**Users need speed**".to_string()])
        );
        assert_eq!(
            projection
                .brief
                .problem_space
                .as_ref()
                .map(|excerpt| excerpt.paragraphs.clone()),
            Some(vec![
                "*Fast feedback loops* reduce coordination drag.".to_string()
            ])
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

        let evidence = projection.evidence.unwrap();
        assert_eq!(
            evidence
                .feasibility
                .as_ref()
                .map(|excerpt| excerpt.paragraphs.clone()),
            Some(vec!["Looks practical.".to_string()])
        );
        assert_eq!(evidence.key_findings, vec!["Existing tools are close."]);
        assert_eq!(evidence.unknowns, vec!["Long-tail migration effort"]);
        assert_eq!(
            evidence.sources,
            vec![
                BearingEvidenceSourceSummary {
                    id: "SRC-01".to_string(),
                    class: "academic".to_string(),
                    provenance: "manual:prior-art-review".to_string(),
                    authority: "high".to_string(),
                    freshness: "medium".to_string(),
                },
                BearingEvidenceSourceSummary {
                    id: "SRC-02".to_string(),
                    class: "web".to_string(),
                    provenance: "manual:official-doc".to_string(),
                    authority: "medium".to_string(),
                    freshness: "high".to_string(),
                },
            ]
        );

        let assessment = projection.assessment.unwrap();
        assert_eq!(
            assessment.recommendation.as_deref(),
            Some("Proceed [SRC-01][SRC-02]")
        );
        assert_eq!(assessment.cited_sources, vec!["SRC-01", "SRC-02"]);
        assert_eq!(
            assessment.findings,
            vec!["Finding supported by evidence [SRC-01]"]
        );
        assert_eq!(
            assessment
                .opportunity_cost
                .as_ref()
                .map(|excerpt| excerpt.paragraphs.clone()),
            Some(vec!["Delayed roadmap item [SRC-02].".to_string()])
        );
        assert_eq!(assessment.dependencies, vec!["Team bandwidth [SRC-02]"]);
        assert_eq!(assessment.alternatives, vec!["Delay and observe"]);
        assert_eq!(projection.frontmatter_datetimes.len(), 2);
    }
}
