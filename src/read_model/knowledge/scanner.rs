//! Knowledge scanner
//!
//! Scans stories, voyages, and ad-hoc files for knowledge.

use std::path::Path;
use std::sync::LazyLock;

use anyhow::Result;
use rayon::prelude::*;
use regex::Regex;
use walkdir::WalkDir;

static KNOWLEDGE_FIELD_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\|\s*\*\*(\w+(?:\s+\w+)?)\*\*\s*\|\s*([^|]*)\|").unwrap());
static KNOWLEDGE_HEADER_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"###\s+(L\d+|ML\d+):\s*(.*)").unwrap());
static SCOPE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?m)^scope:\s*(.+)$").unwrap());

use super::model::{Knowledge, KnowledgeSourceType};

/// Parse a knowledge table from markdown content.
///
/// Knowledge tables have this format:
/// ```markdown
/// ### L001: Title
///
/// | Field | Value |
/// |-------|-------|
/// | **Category** | value |
/// | **Context** | value |
/// | **Insight** | value |
/// | **Suggested Action** | value |
/// | **Applies To** | value |
/// | **Applied** | value |
/// ```
fn parse_knowledge_table(
    content: &str,
    id: &str,
    title: &str,
    header_end: usize,
) -> Option<Knowledge> {
    let remaining = &content[header_end..];

    // Extract table rows (lines with pipes)
    let mut fields = std::collections::HashMap::new();

    let field_re = &*KNOWLEDGE_FIELD_RE;

    for line in remaining.lines() {
        // Stop at next header or section
        if line.starts_with("##") || line.starts_with("---") {
            break;
        }

        // Skip header and separator rows
        if line.contains("---") || line.contains("Field") {
            continue;
        }

        if let Some(caps) = field_re.captures(line) {
            let field_name = caps.get(1)?.as_str().to_lowercase().replace(' ', "");
            let value = caps.get(2)?.as_str().trim().to_string();
            fields.insert(field_name, value);
        }
    }

    // Require at least an insight field
    if !fields.contains_key("insight") {
        return None;
    }

    let observed_at = fields.get("observedat").and_then(|s| {
        chrono::DateTime::parse_from_rfc3339(s)
            .ok()
            .map(|dt| dt.with_timezone(&chrono::Utc))
    });

    let score = fields
        .get("score")
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.5);

    let confidence = fields
        .get("confidence")
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.8);

    Some(Knowledge {
        id: id.to_string(),
        source: std::path::PathBuf::new(),       // Set by caller
        source_type: KnowledgeSourceType::Story, // Set by caller
        scope: None,
        title: title.to_string(),
        category: fields
            .get("category")
            .cloned()
            .unwrap_or_else(|| "unknown".to_string()),
        context: fields.get("context").cloned().unwrap_or_default(),
        insight: fields.get("insight").cloned().unwrap_or_default(),
        suggested_action: fields.get("suggestedaction").cloned().unwrap_or_default(),
        applies_to: fields.get("appliesto").cloned().unwrap_or_default(),
        applied: fields.get("applied").cloned().unwrap_or_default(),
        observed_at,
        score,
        confidence,
    })
}

/// Parse all knowledge from a markdown content section.
fn parse_knowledge_from_content(
    content: &str,
    source: &Path,
    source_type: KnowledgeSourceType,
) -> Vec<Knowledge> {
    let mut knowledge_list = Vec::new();

    let header_re = &*KNOWLEDGE_HEADER_RE;

    for caps in header_re.captures_iter(content) {
        let id = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        let title = caps.get(2).map(|m| m.as_str().trim()).unwrap_or("");
        let header_end = caps.get(0).map(|m| m.end()).unwrap_or(0);

        if let Some(mut knowledge) = parse_knowledge_table(content, id, title, header_end) {
            knowledge.source = source.to_path_buf();
            knowledge.source_type = source_type;
            knowledge_list.push(knowledge);
        }
    }

    knowledge_list
}

/// Extract a section from markdown content.
/// Finds the section starting with the given header and returns content until the next section.
fn extract_section(content: &str, header: &str) -> Option<String> {
    // Find the start of the section
    let header_with_newline = format!("{}\n", header);
    let start_idx = content.find(&header_with_newline)?;
    let content_start = start_idx + header_with_newline.len();

    // Find the end - next ## header or --- or end of content
    let remaining = &content[content_start..];

    // Find the first occurrence of a new section marker
    let end_offset = remaining
        .find("\n## ")
        .or_else(|| remaining.find("\n---"))
        .unwrap_or(remaining.len());

    Some(remaining[..end_offset].to_string())
}

/// Extract scope from story frontmatter
fn extract_scope_from_frontmatter(content: &str) -> Option<String> {
    SCOPE_RE
        .captures(content)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().trim().to_string())
}

/// Scan story bundles for knowledge (only done stories).
fn scan_story_knowledge(board_dir: &Path) -> Vec<Knowledge> {
    let stories_dir = board_dir.join("stories");

    if !stories_dir.exists() {
        return Vec::new();
    }

    // Iterate over story directories (bundles)
    let entries: Vec<_> = std::fs::read_dir(&stories_dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .map(|e| e.path())
        .collect();

    // Parse in parallel
    entries
        .par_iter()
        .flat_map(|bundle_path| {
            let readme_path = bundle_path.join("README.md");
            let reflect_path = bundle_path.join("REFLECT.md");

            if !readme_path.exists() || !reflect_path.exists() {
                return Vec::new();
            }

            let readme_content = match std::fs::read_to_string(&readme_path) {
                Ok(c) => c,
                Err(_) => return Vec::new(),
            };

            // Only scan done stories for knowledge
            // Use simple string check for efficiency, similar to previous implementation
            if !readme_content.contains("status: done") {
                return Vec::new();
            }

            let reflect_content = match std::fs::read_to_string(&reflect_path) {
                Ok(c) => c,
                Err(_) => return Vec::new(),
            };

            let mut knowledge_list = parse_knowledge_from_content(
                &reflect_content,
                &reflect_path,
                KnowledgeSourceType::Story,
            );

            // Extract scope from README frontmatter
            let scope = extract_scope_from_frontmatter(&readme_content);
            for knowledge in &mut knowledge_list {
                knowledge.scope = scope.clone();
            }

            knowledge_list
        })
        .collect()
}

/// Scan voyage KNOWLEDGE.md files.
/// Only scans the "## Synthesis" section to avoid duplicating story knowledge.
fn scan_voyage_knowledge(board_dir: &Path) -> Vec<Knowledge> {
    let epics_dir = board_dir.join("epics");

    if !epics_dir.exists() {
        return Vec::new();
    }

    // Collect all KNOWLEDGE.md files
    let files: Vec<_> = WalkDir::new(&epics_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name() == "KNOWLEDGE.md")
        .map(|e| e.path().to_path_buf())
        .collect();

    // Parse in parallel
    files
        .par_iter()
        .flat_map(|path| {
            let content = match std::fs::read_to_string(path) {
                Ok(c) => c,
                Err(_) => return Vec::new(),
            };

            // Only extract from ## Synthesis section
            let section_content = extract_section(&content, "## Synthesis");
            let Some(section_content) = section_content else {
                return Vec::new();
            };
            parse_knowledge_from_content(&section_content, path, KnowledgeSourceType::Voyage)
        })
        .collect()
}

/// Scan ad-hoc knowledge from docs/knowledge/.
fn scan_adhoc_knowledge(board_dir: &Path) -> Vec<Knowledge> {
    // Go up from board_dir (.keel) to find docs/knowledge
    let knowledge_dir = board_dir
        .parent()
        .map(|p| p.join("knowledge"))
        .unwrap_or_default();

    if !knowledge_dir.exists() {
        return Vec::new();
    }

    // Collect all markdown files
    let files: Vec<_> = WalkDir::new(&knowledge_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
        .map(|e| e.path().to_path_buf())
        .collect();

    // Parse in parallel
    files
        .par_iter()
        .flat_map(|path| {
            let content = match std::fs::read_to_string(path) {
                Ok(c) => c,
                Err(_) => return Vec::new(),
            };

            parse_knowledge_from_content(&content, path, KnowledgeSourceType::Adhoc)
        })
        .collect()
}

/// Scan all knowledge sources and return all knowledge units.
pub fn scan_all_knowledge(board_dir: &Path) -> Result<Vec<Knowledge>> {
    // Run all three scans in parallel using rayon
    let (story, (voyage, adhoc)) = rayon::join(
        || scan_story_knowledge(board_dir),
        || {
            rayon::join(
                || scan_voyage_knowledge(board_dir),
                || scan_adhoc_knowledge(board_dir),
            )
        },
    );

    let mut all = Vec::with_capacity(story.len() + voyage.len() + adhoc.len());
    all.extend(story);
    all.extend(voyage);
    all.extend(adhoc);

    Ok(all)
}

/// Filter knowledge to only unapplied units.
pub fn filter_unapplied(knowledge_list: Vec<Knowledge>) -> Vec<Knowledge> {
    knowledge_list
        .into_iter()
        .filter(|l| l.is_pending())
        .collect()
}

/// Filter knowledge units by category.
pub fn filter_by_category(knowledge_list: Vec<Knowledge>, category: &str) -> Vec<Knowledge> {
    let category_lower = category.to_lowercase();
    knowledge_list
        .into_iter()
        .filter(|l| l.category.to_lowercase() == category_lower)
        .collect()
}

/// Parse the "Applies To" field to extract target file patterns.
pub fn parse_applies_to(applies_to: &str) -> Vec<String> {
    if applies_to.is_empty() || applies_to.to_lowercase().contains("to be determined") {
        return Vec::new();
    }

    applies_to
        .split([',', ';'])
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty() && !s.to_lowercase().contains("none"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_board() -> TempDir {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create board structure
        fs::create_dir_all(root.join("stories")).unwrap();
        fs::create_dir_all(root.join("epics/test-epic/voyages/01-test")).unwrap();

        temp
    }

    #[test]
    fn parse_knowledge_table_extracts_fields() {
        let content = r#"
### L001: Test Knowledge Title

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Some context |
| **Insight** | The actual insight |
| **Suggested Action** | Do something |
| **Applies To** | CLAUDE.md |
| **Applied** | |
| **Observed At** | 2026-02-22T12:00:00Z |
| **Score** | 0.9 |
| **Confidence** | 1.0 |
"#;

        let knowledge_list = parse_knowledge_from_content(
            content,
            Path::new("/test.md"),
            KnowledgeSourceType::Story,
        );

        assert_eq!(knowledge_list.len(), 1);
        let k = &knowledge_list[0];
        assert_eq!(k.id, "L001");
        assert_eq!(k.title, "Test Knowledge Title");
        assert_eq!(k.category, "code");
        assert_eq!(k.context, "Some context");
        assert_eq!(k.insight, "The actual insight");
        assert_eq!(k.suggested_action, "Do something");
        assert_eq!(k.applies_to, "CLAUDE.md");
        assert!(k.applied.is_empty());
        assert!(k.observed_at.is_some());
        assert_eq!(k.score, 0.9);
        assert_eq!(k.confidence, 1.0);
    }

    #[test]
    fn parse_multiple_knowledge() {
        let content = r#"
### L001: First Knowledge

| Field | Value |
|-------|-------|
| **Category** | code |
| **Insight** | First insight |

### L002: Second Knowledge

| Field | Value |
|-------|-------|
| **Category** | process |
| **Insight** | Second insight |
"#;

        let knowledge_list = parse_knowledge_from_content(
            content,
            Path::new("/test.md"),
            KnowledgeSourceType::Story,
        );

        assert_eq!(knowledge_list.len(), 2);
        assert_eq!(knowledge_list[0].id, "L001");
        assert_eq!(knowledge_list[1].id, "L002");
    }

    #[test]
    fn parse_voyage_knowledge_id() {
        let content = r#"
### ML001: Voyage Knowledge

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Insight** | Voyage insight |
"#;

        let knowledge_list = parse_knowledge_from_content(
            content,
            Path::new("/test.md"),
            KnowledgeSourceType::Voyage,
        );

        assert_eq!(knowledge_list.len(), 1);
        assert_eq!(knowledge_list[0].id, "ML001");
    }

    #[test]
    fn skip_knowledge_without_insight() {
        let content = r#"
### L001: Missing Insight

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Some context |
"#;

        let knowledge_list = parse_knowledge_from_content(
            content,
            Path::new("/test.md"),
            KnowledgeSourceType::Story,
        );

        assert!(knowledge_list.is_empty());
    }

    #[test]
    fn scan_story_knowledge_from_done_stage() {
        let temp = create_test_board();

        // Create a story bundle
        let bundle_dir = temp.path().join("stories/FEAT0001");
        fs::create_dir_all(&bundle_dir).unwrap();

        let readme_content = r#"---
id: FEAT0001
title: Test Story
status: done
---
"#;
        let reflect_content = r#"# Reflection

## Knowledge

### L001: Test Insight

| Field | Value |
|-------|-------|
| **Category** | testing |
| **Insight** | This is a test insight |
"#;

        fs::write(bundle_dir.join("README.md"), readme_content).unwrap();
        fs::write(bundle_dir.join("REFLECT.md"), reflect_content).unwrap();

        let knowledge_list = scan_story_knowledge(temp.path());

        assert_eq!(knowledge_list.len(), 1);
        assert_eq!(knowledge_list[0].id, "L001");
        assert_eq!(knowledge_list[0].category, "testing");
        assert_eq!(knowledge_list[0].source_type, KnowledgeSourceType::Story);
    }

    #[test]
    fn scan_voyage_knowledge_from_synthesis_section() {
        let temp = create_test_board();

        // Create a voyage KNOWLEDGE.md
        let knowledge_content = r#"# Voyage Knowledge

## Story Knowledge

### From FEAT0001
(should be ignored)

## Synthesis

### ML001: Synthesized Knowledge

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Insight** | A synthesized insight |
"#;

        fs::write(
            temp.path()
                .join("epics/test-epic/voyages/01-test/KNOWLEDGE.md"),
            knowledge_content,
        )
        .unwrap();

        let knowledge_list = scan_voyage_knowledge(temp.path());

        assert_eq!(knowledge_list.len(), 1);
        assert_eq!(knowledge_list[0].id, "ML001");
        assert_eq!(knowledge_list[0].source_type, KnowledgeSourceType::Voyage);
    }

    #[test]
    fn filter_unapplied_removes_applied() {
        let knowledge_list = vec![
            Knowledge {
                id: "L001".to_string(),
                source: Path::new("/a.md").to_path_buf(),
                source_type: KnowledgeSourceType::Story,
                scope: None,
                title: "Applied".to_string(),
                category: "code".to_string(),
                context: String::new(),
                insight: "Insight".to_string(),
                suggested_action: String::new(),
                applies_to: String::new(),
                applied: "file.md (2026-01-20)".to_string(),
                observed_at: None,
                score: 0.5,
                confidence: 0.8,
            },
            Knowledge {
                id: "L002".to_string(),
                source: Path::new("/b.md").to_path_buf(),
                source_type: KnowledgeSourceType::Story,
                scope: None,
                title: "Pending".to_string(),
                category: "code".to_string(),
                context: String::new(),
                insight: "Insight".to_string(),
                suggested_action: String::new(),
                applies_to: String::new(),
                applied: String::new(),
                observed_at: None,
                score: 0.5,
                confidence: 0.8,
            },
        ];

        let filtered = filter_unapplied(knowledge_list);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id, "L002");
    }

    #[test]
    fn filter_by_category_case_insensitive() {
        let knowledge_list = vec![
            Knowledge {
                id: "L001".to_string(),
                source: Path::new("/a.md").to_path_buf(),
                source_type: KnowledgeSourceType::Story,
                scope: None,
                title: "Code".to_string(),
                category: "Code".to_string(),
                context: String::new(),
                insight: "Insight".to_string(),
                suggested_action: String::new(),
                applies_to: String::new(),
                applied: String::new(),
                observed_at: None,
                score: 0.5,
                confidence: 0.8,
            },
            Knowledge {
                id: "L002".to_string(),
                source: Path::new("/b.md").to_path_buf(),
                source_type: KnowledgeSourceType::Story,
                scope: None,
                title: "Process".to_string(),
                category: "process".to_string(),
                context: String::new(),
                insight: "Insight".to_string(),
                suggested_action: String::new(),
                applies_to: String::new(),
                applied: String::new(),
                observed_at: None,
                score: 0.5,
                confidence: 0.8,
            },
        ];

        let filtered = filter_by_category(knowledge_list, "code");
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id, "L001");
    }

    #[test]
    fn parse_applies_to_single() {
        assert_eq!(parse_applies_to("CLAUDE.md"), vec!["CLAUDE.md"]);
    }

    #[test]
    fn parse_applies_to_multiple() {
        assert_eq!(
            parse_applies_to("CLAUDE.md, CONVENTIONS.md"),
            vec!["CLAUDE.md", "CONVENTIONS.md"]
        );
    }

    #[test]
    fn parse_applies_to_to_be_determined() {
        assert!(parse_applies_to("To be determined").is_empty());
        assert!(parse_applies_to("(to be determined)").is_empty());
    }

    #[test]
    fn parse_applies_to_empty() {
        assert!(parse_applies_to("").is_empty());
    }

    #[test]
    fn parse_applies_to_none() {
        assert!(parse_applies_to("None").is_empty());
        assert!(parse_applies_to("none").is_empty());
    }

    #[test]
    fn extract_scope_from_frontmatter_finds_scope() {
        let content = r#"---
id: FEAT0001
scope: board-cli/07-knowledge-integration
status: done
---
"#;

        assert_eq!(
            extract_scope_from_frontmatter(content),
            Some("board-cli/07-knowledge-integration".to_string())
        );
    }

    #[test]
    fn extract_scope_from_frontmatter_returns_none_when_missing() {
        let content = r#"---
id: FEAT0001
status: done
---
"#;

        assert!(extract_scope_from_frontmatter(content).is_none());
    }
}
