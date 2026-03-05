//! Show bearing command.

use anyhow::Result;
use owo_colors::OwoColorize;
use regex::{Captures, Regex};
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::sync::LazyLock;

use crate::cli::presentation::show::{ShowDocument, ShowKeyValues, ShowSection};
use crate::domain::model::BearingStatus;
use crate::infrastructure::config::{find_board_dir, load_config};
use crate::infrastructure::loader::load_board;
use crate::infrastructure::scoring::{calculate_score, load_assessment};

use super::guidance::{informational_for_show, print_human};
use super::{classify_fog, format_factor};

const BRIEF_PLACEHOLDER: &str = "(not authored in BRIEF.md yet)";
const SURVEY_PLACEHOLDER: &str = "(not authored in SURVEY.md yet)";
const ASSESSMENT_PLACEHOLDER: &str = "(not authored in ASSESSMENT.md yet)";
const NONE_PLACEHOLDER: &str = "(none)";
static STRONG_EMPHASIS_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\*\*([^*]+)\*\*").expect("valid strong emphasis regex"));
static EMPHASIS_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\*([^*]+)\*").expect("valid emphasis regex"));

/// Show detailed bearing information.
pub fn run(pattern: &str) -> Result<()> {
    let board_dir = find_board_dir()?;
    let board = load_board(&board_dir)?;
    let (config, _source) = load_config();
    let weights = config.current_weights();
    let bearing = board.require_bearing(pattern)?;
    let bearing_dir = bearing.path.parent().unwrap_or(&bearing.path);
    let readme_content = fs::read_to_string(&bearing.path).unwrap_or_default();
    let brief_path = bearing_dir.join("BRIEF.md");
    let survey_path = bearing_dir.join("SURVEY.md");
    let assessment_path = bearing_dir.join("ASSESSMENT.md");
    let brief_content = fs::read_to_string(&brief_path).unwrap_or_default();
    let survey_content = fs::read_to_string(&survey_path).ok();
    let assessment_content = fs::read_to_string(&assessment_path).ok();
    let width = crate::cli::presentation::terminal::get_terminal_width();

    let mut metadata = ShowKeyValues::new()
        .with_min_label_width(9)
        .row("Title:", format!("{}", bearing.frontmatter.title.bold()))
        .row("Status:", bearing.frontmatter.status.to_string());

    if matches!(
        bearing.frontmatter.status,
        BearingStatus::Exploring | BearingStatus::Parked
    ) {
        let fog = classify_fog(&board_dir, bearing);
        metadata.push_row("Fog:", fog.as_banner().to_string());
    }
    metadata.push_optional_row(
        "Created:",
        bearing
            .frontmatter
            .created_at
            .map(|created_at| format!("{}", created_at.dimmed())),
    );
    metadata.push_optional_row(
        "Laid:",
        bearing
            .frontmatter
            .laid_at
            .map(|laid_at| format!("{}", laid_at.dimmed())),
    );
    let mut skipped_date_keys = BTreeSet::from(["created_at".to_string()]);
    if bearing.frontmatter.laid_at.is_some() {
        skipped_date_keys.insert("laid_at".to_string());
    }
    for (key, value) in extract_frontmatter_datetime_fields(&readme_content) {
        if skipped_date_keys.contains(&key) {
            continue;
        }
        metadata.push_row(
            format!("{}:", datetime_field_label(&key)),
            format!("{}", value.dimmed()),
        );
    }
    metadata.push_row("Path:", format!("{}", bearing.path.display().dimmed()));

    let mut sections = vec![
        render_documents_section(bearing),
        render_brief_section(&brief_content),
        render_survey_section(survey_content.as_deref()),
        render_assessment_section(
            &assessment_path,
            assessment_content.as_deref(),
            config.mode(),
            &weights,
        ),
    ];

    if let Some(reason) = &bearing.frontmatter.decline_reason {
        let mut decline = ShowSection::new("Decline Reason");
        decline.push_lines([reason.to_string()]);
        sections.push(decline);
    }

    let mut document = ShowDocument::new();
    document.push_header(metadata, Some(width));
    document.push_sections_spaced(sections);
    document.print();
    print_human(informational_for_show().as_ref());

    Ok(())
}

fn render_documents_section(bearing: &crate::domain::model::Bearing) -> ShowSection {
    let mut section = ShowSection::new("Documents");
    let mut fields = ShowKeyValues::new().with_indent(2).with_min_label_width(15);
    fields.push_row("README.md:", format!("{}", "authored".green()));
    fields.push_row("BRIEF.md:", format!("{}", "authored".green()));
    fields.push_row(
        "SURVEY.md:",
        if bearing.has_survey {
            format!("{}", "authored".green())
        } else {
            format!("{}", "not created".dimmed())
        },
    );
    fields.push_row(
        "ASSESSMENT.md:",
        if bearing.has_assessment {
            format!("{}", "authored".green())
        } else {
            format!("{}", "not created".dimmed())
        },
    );
    section.push_key_values(fields);
    section
}

fn render_brief_section(content: &str) -> ShowSection {
    let hypothesis = extract_markdown_section(content, "## Hypothesis");
    let problem_space = extract_markdown_section(content, "## Problem Space");
    let success_criteria =
        parse_checkbox_items(&extract_markdown_section(content, "## Success Criteria"));
    let open_questions =
        parse_markdown_list_items(&extract_markdown_section(content, "## Open Questions"));

    let checked_criteria = success_criteria
        .iter()
        .filter(|(checked, _)| *checked)
        .count();
    let unchecked_criteria: Vec<String> = success_criteria
        .iter()
        .filter(|(checked, _)| !checked)
        .map(|(_, item)| item.clone())
        .collect();

    let mut section = ShowSection::new("Brief");
    push_labeled_text_block(
        &mut section,
        "Hypothesis:",
        first_authored_line(&hypothesis).unwrap_or_else(|| BRIEF_PLACEHOLDER.dimmed().to_string()),
    );
    push_labeled_text_block(
        &mut section,
        "Problem Space:",
        first_authored_line(&problem_space)
            .unwrap_or_else(|| BRIEF_PLACEHOLDER.dimmed().to_string()),
    );

    let mut fields = ShowKeyValues::new().with_indent(2).with_min_label_width(18);
    if success_criteria.is_empty() {
        fields.push_row(
            "Success Criteria:",
            format!("{}", BRIEF_PLACEHOLDER.dimmed()),
        );
    } else {
        fields.push_row(
            "Success Criteria:",
            format!("{checked_criteria}/{} checked", success_criteria.len()),
        );
    }
    fields.push_row("Open Questions:", open_questions.len().to_string());
    section.push_key_values(fields);

    section.push_labeled_bullets(
        "Open Questions:",
        open_questions
            .iter()
            .map(|item| render_inline_markdown(item))
            .collect::<Vec<_>>(),
        Some(format!("{}", NONE_PLACEHOLDER.dimmed())),
    );
    section.push_labeled_bullets(
        "Unchecked Criteria:",
        unchecked_criteria
            .iter()
            .map(|item| render_inline_markdown(item))
            .collect::<Vec<_>>(),
        Some(format!("{}", NONE_PLACEHOLDER.dimmed())),
    );
    section
}

fn render_survey_section(content: Option<&str>) -> ShowSection {
    let mut section = ShowSection::new("Survey");
    let Some(content) = content else {
        section.push_lines([format!("  {}", SURVEY_PLACEHOLDER.dimmed())]);
        return section;
    };

    let feasibility = extract_markdown_section(content, "### Feasibility");
    let key_findings =
        parse_markdown_list_items(&extract_markdown_section(content, "## Key Findings"));
    let unknowns = parse_markdown_list_items(&extract_markdown_section(content, "## Unknowns"));

    push_labeled_text_block(
        &mut section,
        "Feasibility:",
        first_authored_line(&feasibility)
            .unwrap_or_else(|| SURVEY_PLACEHOLDER.dimmed().to_string()),
    );

    let mut fields = ShowKeyValues::new().with_indent(2).with_min_label_width(13);
    fields.push_row("Key Findings:", key_findings.len().to_string());
    fields.push_row("Unknowns:", unknowns.len().to_string());
    section.push_key_values(fields);

    section.push_labeled_bullets(
        "Key Findings:",
        key_findings
            .iter()
            .map(|item| render_inline_markdown(item))
            .collect::<Vec<_>>(),
        Some(format!("{}", NONE_PLACEHOLDER.dimmed())),
    );
    section.push_labeled_bullets(
        "Unknowns:",
        unknowns
            .iter()
            .map(|item| render_inline_markdown(item))
            .collect::<Vec<_>>(),
        Some(format!("{}", NONE_PLACEHOLDER.dimmed())),
    );
    section
}

fn render_assessment_section(
    assessment_path: &Path,
    assessment_content: Option<&str>,
    mode: &str,
    weights: &crate::infrastructure::config::ModeWeights,
) -> ShowSection {
    let mut section = ShowSection::new("Assessment");
    let Some(content) = assessment_content else {
        section.push_lines([format!("  {}", ASSESSMENT_PLACEHOLDER.dimmed())]);
        return section;
    };

    let mut fields = ShowKeyValues::new().with_indent(2).with_min_label_width(17);
    fields.push_row("Status:", format!("{}", "authored".green()));
    fields.push_row("Mode:", mode.to_string());

    if let Ok(factors) = load_assessment(assessment_path) {
        fields.push_row("Impact:", format_factor(factors.impact));
        fields.push_row("Confidence:", format_factor(factors.confidence));
        fields.push_row("Effort:", format_factor(factors.effort));
        fields.push_row("Risk:", format_factor(factors.risk));
        if factors.is_complete() {
            if let Ok(score) = calculate_score(&factors, weights) {
                fields.push_row("EV Score:", format!("{:.2}", score.weighted_score));
            }
        } else {
            fields.push_row("Missing Factors:", factors.missing_factors().join(", "));
        }
    }

    let recommendation_section = extract_markdown_section(content, "## Recommendation");
    fields.push_optional_row(
        "Recommendation:",
        extract_checked_recommendation(&recommendation_section)
            .map(|item| render_inline_markdown(&item)),
    );

    let opportunity_cost = extract_markdown_section(content, "### Opportunity Cost");
    let dependencies =
        parse_markdown_list_items(&extract_markdown_section(content, "### Dependencies"));
    let alternatives = parse_markdown_list_items(&extract_markdown_section(
        content,
        "### Alternatives Considered",
    ));
    section.push_key_values(fields);
    push_labeled_text_block(
        &mut section,
        "Opportunity Cost:",
        first_authored_line(&opportunity_cost)
            .unwrap_or_else(|| format!("{}", NONE_PLACEHOLDER.dimmed())),
    );

    let mut counts = ShowKeyValues::new().with_indent(2).with_min_label_width(17);
    counts.push_row("Dependencies:", dependencies.len().to_string());
    counts.push_row("Alternatives:", alternatives.len().to_string());
    section.push_key_values(counts);

    section.push_labeled_bullets(
        "Dependencies:",
        dependencies
            .iter()
            .map(|item| render_inline_markdown(item))
            .collect::<Vec<_>>(),
        Some(format!("{}", NONE_PLACEHOLDER.dimmed())),
    );
    section.push_labeled_bullets(
        "Alternatives:",
        alternatives
            .iter()
            .map(|item| render_inline_markdown(item))
            .collect::<Vec<_>>(),
        Some(format!("{}", NONE_PLACEHOLDER.dimmed())),
    );
    section
}

fn first_authored_line(section: &str) -> Option<String> {
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

fn push_labeled_text_block(section: &mut ShowSection, label: &str, value: String) {
    section.push_lines([format!("  {label}")]);

    let mut value_lines: Vec<String> = value
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| format!("    {}", render_inline_markdown(line)))
        .collect();

    if value_lines.is_empty() {
        value_lines.push(format!("    {}", NONE_PLACEHOLDER.dimmed()));
    }

    section.push_lines(value_lines);
}

fn render_inline_markdown(value: &str) -> String {
    let strong_to_italic = STRONG_EMPHASIS_RE.replace_all(value, |captures: &Captures<'_>| {
        format!("{}", captures[1].to_string().italic())
    });
    EMPHASIS_RE
        .replace_all(&strong_to_italic, |captures: &Captures<'_>| {
            format!("{}", captures[1].to_string().italic())
        })
        .into_owned()
}

fn extract_markdown_section(content: &str, heading: &str) -> String {
    let mut in_section = false;
    let mut result = String::new();
    let heading_level = heading.chars().take_while(|ch| *ch == '#').count();

    for line in content.lines() {
        if line.trim() == heading {
            in_section = true;
            continue;
        }
        if in_section {
            if line.starts_with('#') {
                let level = line.chars().take_while(|ch| *ch == '#').count();
                if level <= heading_level {
                    break;
                }
            }
            result.push_str(line);
            result.push('\n');
        }
    }

    result.trim().to_string()
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

fn extract_frontmatter_datetime_fields(content: &str) -> Vec<(String, String)> {
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
                fields.push((key.to_string(), value.to_string()));
            }
        }
    }

    fields.sort_by(|a, b| a.0.cmp(&b.0));
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
                ("created_at".to_string(), "2026-03-05T10:00:00".to_string()),
                ("laid_at".to_string(), "2026-03-06T10:00:00".to_string()),
                ("surveyed_at".to_string(), "2026-03-07T10:00:00".to_string()),
            ]
        );
    }

    #[test]
    fn extract_markdown_section_stops_at_same_or_higher_heading_level() {
        let content = r#"
## Root

### One
one

### Two
two

## Next
next
"#;

        assert_eq!(extract_markdown_section(content, "### One"), "one");
        assert_eq!(
            extract_markdown_section(content, "## Root"),
            "### One\none\n\n### Two\ntwo"
        );
    }

    #[test]
    fn render_inline_markdown_replaces_markers_with_italic_styling() {
        let rendered = render_inline_markdown("**Content** and *focus*");
        assert!(!rendered.contains("**Content**"));
        assert!(!rendered.contains("*focus*"));
        assert!(rendered.contains("Content"));
        assert!(rendered.contains("focus"));
    }
}
