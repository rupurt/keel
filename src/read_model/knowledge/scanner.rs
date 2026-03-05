//! Knowledge scanner
//!
//! Scans stories, voyages, and ad-hoc files for knowledge.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use anyhow::{Context, Result, bail};
use chrono::{NaiveDateTime, Utc};
use rayon::prelude::*;
use regex::Regex;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

static KNOWLEDGE_FIELD_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\|\s*\*\*(\w+(?:\s+\w+)*)\*\*\s*\|\s*([^|]*)\|").unwrap());
static KNOWLEDGE_HEADER_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^###\s+([A-Za-z0-9]{9}|L\d+|ML\d+):\s*(.*)$").unwrap());
static KNOWLEDGE_HEADER_LINE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\s*###\s+)([A-Za-z0-9]{9}|L\d+|ML\d+)(:\s*.*)$").unwrap());
static KNOWLEDGE_LINK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?m)^\s*[-*]\s+\[([A-Za-z0-9]{9})\]\(([^)]+)\)"#).unwrap());
static SCOPE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?m)^scope:\s*(.+)$").unwrap());
static LEGACY_KNOWLEDGE_ID_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(L\d+|ML\d+)$").unwrap());

use super::model::{Knowledge, KnowledgeSourceType};

/// Validation issue found in a knowledge block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnowledgeValidationIssue {
    pub id: String,
    pub title: String,
    pub reason: String,
}

/// In-memory snapshot of the canonical knowledge catalog.
#[derive(Debug, Clone)]
pub struct KnowledgeCatalog {
    pub generated_at: chrono::DateTime<Utc>,
    pub units: Vec<Knowledge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct KnowledgeFileFrontmatter {
    source_type: KnowledgeSourceType,
    source: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    scope: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    source_story_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::domain::model::deserialize_strict_datetime"
    )]
    created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct KnowledgeArtifactFrontmatter {
    #[serde(
        default,
        deserialize_with = "crate::domain::model::deserialize_strict_datetime"
    )]
    created_at: Option<NaiveDateTime>,
}

/// One-time migration result for canonicalizing knowledge IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KnowledgeMigrationReport {
    pub files_updated: usize,
    pub ids_regenerated: usize,
    pub knowledge_entries: usize,
}

/// Sort mode for `knowledge list`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KnowledgeSort {
    #[default]
    Id,
    Story,
}

pub fn is_canonical_knowledge_id(id: &str) -> bool {
    id.len() == 9 && id.chars().all(|ch| ch.is_ascii_alphanumeric())
}

/// Relevance projection used for context surfacing in execution/planning flows.
#[derive(Debug, Clone)]
pub struct RankedKnowledge {
    pub knowledge: Knowledge,
    pub score: f64,
    pub scope_match: f64,
    pub recency: f64,
}

fn parse_linked_ids(value: &str) -> Vec<String> {
    value
        .split([',', ';'])
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn canonicalize_source_story_id(source: &Path) -> Option<String> {
    let file_name = source.file_name()?.to_str()?;
    if file_name != "REFLECT.md" {
        return None;
    }

    let story_dir = source.parent()?;
    let stories_dir = story_dir.parent()?;
    if stories_dir.file_name()?.to_str()? != "stories" {
        return None;
    }

    story_dir
        .file_name()
        .and_then(|name| name.to_str())
        .map(ToOwned::to_owned)
}

fn normalize_similarity_fields(units: &mut [Knowledge]) {
    let tokenized: Vec<HashSet<String>> = units
        .iter()
        .map(|unit| {
            format!("{} {}", unit.title, unit.insight)
                .split_whitespace()
                .map(|token| {
                    token
                        .trim_matches(|ch: char| !ch.is_ascii_alphanumeric())
                        .to_ascii_lowercase()
                })
                .filter(|token| token.len() >= 3)
                .collect()
        })
        .collect();

    for idx in 0..units.len() {
        let mut best_id: Option<String> = None;
        let mut best_score = 0.0;
        for other_idx in 0..units.len() {
            if idx == other_idx {
                continue;
            }

            let left = &tokenized[idx];
            let right = &tokenized[other_idx];
            if left.is_empty() || right.is_empty() {
                continue;
            }

            let intersection = left.intersection(right).count() as f64;
            let union = left.union(right).count() as f64;
            if union <= 0.0 {
                continue;
            }

            let score = intersection / union;
            if score > best_score {
                best_score = score;
                best_id = Some(units[other_idx].id.clone());
            }
        }

        if let Some(id) = best_id {
            units[idx].similar_to = Some(id);
            units[idx].similarity_score = Some((best_score * 100.0).round() / 100.0);
        } else {
            units[idx].similar_to = None;
            units[idx].similarity_score = None;
        }
    }
}

fn normalize_units(units: &mut [Knowledge]) {
    for unit in units {
        if unit.source_story_id.is_none() {
            unit.source_story_id = canonicalize_source_story_id(&unit.source);
        }
    }
}

fn duplicate_ids(units: &[Knowledge]) -> Vec<String> {
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for unit in units {
        *counts.entry(unit.id.as_str()).or_insert(0) += 1;
    }

    let mut duplicates: Vec<String> = counts
        .into_iter()
        .filter_map(|(id, count)| (count > 1).then_some(id.to_string()))
        .collect();
    duplicates.sort();
    duplicates
}

pub fn knowledge_dir(board_dir: &Path) -> PathBuf {
    board_dir.join("knowledge")
}

pub fn knowledge_file_path(board_dir: &Path, id: &str) -> PathBuf {
    knowledge_dir(board_dir).join(format!("{id}.md"))
}

fn source_path_for_storage(board_dir: &Path, source: &Path) -> String {
    source
        .strip_prefix(board_dir)
        .unwrap_or(source)
        .to_string_lossy()
        .replace('\\', "/")
}

fn source_path_from_storage(board_dir: &Path, stored: &str) -> PathBuf {
    let stored_path = Path::new(stored);
    if stored_path.is_absolute() {
        stored_path.to_path_buf()
    } else {
        board_dir.join(stored_path)
    }
}

fn optional_frontmatter<T: DeserializeOwned>(content: &str) -> Option<T> {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return None;
    }

    crate::infrastructure::parser::parse_frontmatter::<T>(trimmed)
        .ok()
        .map(|(frontmatter, _)| frontmatter)
}

fn content_created_at(content: &str) -> Option<NaiveDateTime> {
    optional_frontmatter::<KnowledgeArtifactFrontmatter>(content).and_then(|fm| fm.created_at)
}

fn apply_created_at(units: &mut [Knowledge], created_at: Option<NaiveDateTime>) {
    let Some(created_at) = created_at else {
        return;
    };

    for unit in units {
        unit.created_at.get_or_insert(created_at);
    }
}

fn parse_knowledge_with_metadata(
    content: &str,
    source: &Path,
    source_type: KnowledgeSourceType,
    created_at: Option<NaiveDateTime>,
) -> Vec<Knowledge> {
    let mut units = parse_knowledge_from_content(content, source, source_type);
    apply_created_at(&mut units, created_at);
    units
}

fn parse_reflection_inline_knowledge(
    reflect_content: &str,
    reflect_path: &Path,
    scope: Option<String>,
    source_story_id: Option<String>,
) -> Vec<Knowledge> {
    let mut inline = parse_knowledge_with_metadata(
        reflect_content,
        reflect_path,
        KnowledgeSourceType::Story,
        content_created_at(reflect_content),
    );
    apply_story_context(&mut inline, scope, source_story_id);
    inline
}

fn render_knowledge_markdown(board_dir: &Path, knowledge: &Knowledge) -> Result<String> {
    let frontmatter = KnowledgeFileFrontmatter {
        source_type: knowledge.source_type,
        source: source_path_for_storage(board_dir, &knowledge.source),
        scope: knowledge.scope.clone(),
        source_story_id: knowledge.source_story_id.clone(),
        created_at: knowledge.created_at,
    };

    Ok(format!(
        "---\n{}---\n\n### {}: {}\n\n| Field | Value |\n|-------|-------|\n| **Category** | {} |\n| **Context** | {} |\n| **Insight** | {} |\n| **Suggested Action** | {} |\n| **Applies To** | {} |\n| **Linked Knowledge IDs** | {} |\n| **Observed At** | {} |\n| **Score** | {:.2} |\n| **Confidence** | {:.2} |\n| **Applied** | {} |\n",
        serde_yaml::to_string(&frontmatter)?,
        knowledge.id,
        knowledge.title,
        knowledge.category,
        knowledge.context,
        knowledge.insight,
        knowledge.suggested_action,
        knowledge.applies_to,
        knowledge.linked_ids.join(", "),
        knowledge
            .observed_at
            .map(|value| value.to_rfc3339())
            .unwrap_or_default(),
        knowledge.score,
        knowledge.confidence,
        knowledge.applied,
    ))
}

fn load_knowledge_file(board_dir: &Path, path: &Path) -> Result<Knowledge> {
    let content =
        fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))?;
    let (frontmatter, body): (KnowledgeFileFrontmatter, _) =
        crate::infrastructure::parser::parse_frontmatter(&content)
            .with_context(|| format!("Failed to parse frontmatter in {}", path.display()))?;
    let mut units = parse_knowledge_from_content(body, path, frontmatter.source_type);
    if units.len() != 1 {
        bail!(
            "knowledge file {} must contain exactly one knowledge block",
            path.display()
        );
    }

    let mut unit = units.remove(0);
    unit.source = source_path_from_storage(board_dir, &frontmatter.source);
    unit.source_type = frontmatter.source_type;
    unit.scope = frontmatter.scope;
    unit.created_at = frontmatter.created_at;
    unit.source_story_id = frontmatter
        .source_story_id
        .or_else(|| canonicalize_source_story_id(&unit.source));
    Ok(unit)
}

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
) -> Result<Knowledge, String> {
    let remaining = &content[header_end..];

    // Extract table rows (lines with pipes)
    let mut fields = HashMap::new();

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
            let Some(field_match) = caps.get(1) else {
                continue;
            };
            let Some(value_match) = caps.get(2) else {
                continue;
            };
            let field_name = field_match.as_str().to_lowercase().replace(' ', "");
            let value = value_match.as_str().trim().to_string();
            fields.insert(field_name, value);
        }
    }

    let mut missing = Vec::new();

    if title.trim().is_empty() || is_placeholder_title(title) {
        missing.push("title");
    }

    let insight = fields.get("insight").cloned().unwrap_or_default();
    if insight.trim().is_empty() {
        missing.push("insight");
    }

    let suggested_action = fields.get("suggestedaction").cloned().unwrap_or_default();
    if suggested_action.trim().is_empty() {
        missing.push("suggested action");
    }

    if !missing.is_empty() {
        return Err(format!(
            "missing or default required fields: {}",
            missing.join(", ")
        ));
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
    let linked_ids = fields
        .get("linkedknowledgeids")
        .map(|raw| parse_linked_ids(raw))
        .unwrap_or_default();

    Ok(Knowledge {
        id: id.to_string(),
        source: std::path::PathBuf::new(),       // Set by caller
        source_type: KnowledgeSourceType::Story, // Set by caller
        scope: None,
        source_story_id: None,
        title: title.to_string(),
        category: fields
            .get("category")
            .cloned()
            .unwrap_or_else(|| "unknown".to_string()),
        context: fields.get("context").cloned().unwrap_or_default(),
        insight,
        suggested_action,
        applies_to: fields.get("appliesto").cloned().unwrap_or_default(),
        applied: fields.get("applied").cloned().unwrap_or_default(),
        created_at: None,
        observed_at,
        score,
        confidence,
        linked_ids,
        similar_to: None,
        similarity_score: None,
    })
}

fn strip_html_comments(content: &str) -> String {
    let mut stripped = content.to_string();
    while let Some(start) = stripped.find("<!--") {
        if let Some(end) = stripped[start..].find("-->") {
            stripped.replace_range(start..start + end + 3, "");
        } else {
            break;
        }
    }
    stripped
}

fn is_placeholder_title(title: &str) -> bool {
    let normalized = title.trim().to_lowercase();
    matches!(
        normalized.as_str(),
        "title" | "todo: title" | "todo:title" | "implementation insight"
    )
}

/// Parse all knowledge from a markdown content section.
pub fn parse_knowledge_from_content(
    content: &str,
    source: &Path,
    source_type: KnowledgeSourceType,
) -> Vec<Knowledge> {
    let mut knowledge_list = Vec::new();
    let sanitized = strip_html_comments(content);

    let header_re = &*KNOWLEDGE_HEADER_RE;

    for caps in header_re.captures_iter(&sanitized) {
        let id = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        let title = caps.get(2).map(|m| m.as_str().trim()).unwrap_or("");
        let header_end = caps.get(0).map(|m| m.end()).unwrap_or(0);

        if let Ok(mut knowledge) = parse_knowledge_table(&sanitized, id, title, header_end) {
            knowledge.source = source.to_path_buf();
            knowledge.source_type = source_type;
            knowledge.source_story_id = canonicalize_source_story_id(source);
            knowledge_list.push(knowledge);
        }
    }

    knowledge_list
}

/// Validate all knowledge blocks in content and return quality issues.
///
/// Rules:
/// - Title must be non-empty and not a scaffold/default label.
/// - Insight must be non-empty.
/// - Suggested Action must be non-empty.
pub fn validate_knowledge_content(content: &str) -> Vec<KnowledgeValidationIssue> {
    let mut issues = Vec::new();
    let sanitized = strip_html_comments(content);
    let header_re = &*KNOWLEDGE_HEADER_RE;
    let mut seen = HashSet::new();

    for caps in header_re.captures_iter(&sanitized) {
        let id = caps.get(1).map(|m| m.as_str()).unwrap_or("").to_string();
        let title = caps
            .get(2)
            .map(|m| m.as_str().trim())
            .unwrap_or("")
            .to_string();
        let header_end = caps.get(0).map(|m| m.end()).unwrap_or(0);

        if !is_canonical_knowledge_id(&id) {
            let reason = if LEGACY_KNOWLEDGE_ID_RE.is_match(&id) {
                format!(
                    "knowledge id '{}' uses legacy local format; use canonical 9-character generated IDs",
                    id
                )
            } else {
                format!(
                    "knowledge id '{}' must be a canonical 9-character generated ID",
                    id
                )
            };
            issues.push(KnowledgeValidationIssue {
                id: id.clone(),
                title: title.clone(),
                reason,
            });
        }

        if !seen.insert(id.clone()) {
            issues.push(KnowledgeValidationIssue {
                id: id.clone(),
                title: title.clone(),
                reason: format!("duplicate knowledge id '{}' in the same document", id),
            });
        }

        if let Err(reason) = parse_knowledge_table(&sanitized, &id, &title, header_end) {
            issues.push(KnowledgeValidationIssue { id, title, reason });
        }
    }

    issues
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

fn extract_reflection_knowledge_links(content: &str) -> Vec<(String, String)> {
    let sanitized = strip_html_comments(content);
    KNOWLEDGE_LINK_RE
        .captures_iter(&sanitized)
        .filter_map(|caps| {
            let id = caps.get(1)?.as_str().to_string();
            let target = caps.get(2)?.as_str().to_string();
            Some((id, target))
        })
        .collect()
}

fn load_reflection_linked_knowledge(
    board_dir: &Path,
    reflect_path: &Path,
    content: &str,
) -> Result<Vec<Knowledge>> {
    extract_reflection_knowledge_links(content)
        .into_iter()
        .map(|(id, target)| {
            let knowledge_path = reflect_path
                .parent()
                .unwrap_or(board_dir)
                .join(target)
                .components()
                .collect::<PathBuf>();
            let knowledge = load_knowledge_file(board_dir, &knowledge_path).with_context(|| {
                format!(
                    "Failed to load linked knowledge '{}' from {}",
                    id,
                    reflect_path.display()
                )
            })?;
            if knowledge.id != id {
                bail!(
                    "reflection link in {} points to knowledge '{}' but file contains '{}'",
                    reflect_path.display(),
                    id,
                    knowledge.id
                );
            }
            Ok(knowledge)
        })
        .collect()
}

fn apply_story_context(
    units: &mut [Knowledge],
    scope: Option<String>,
    source_story_id: Option<String>,
) {
    for knowledge in units {
        knowledge.scope = scope.clone().or_else(|| knowledge.scope.clone());
        knowledge.source_story_id = source_story_id
            .clone()
            .or_else(|| knowledge.source_story_id.clone());
    }
}

fn dedupe_by_id(units: Vec<Knowledge>) -> Vec<Knowledge> {
    let mut deduped = HashMap::new();
    for unit in units {
        deduped.entry(unit.id.clone()).or_insert(unit);
    }

    let mut units: Vec<_> = deduped.into_values().collect();
    units.sort_by(|a, b| a.id.cmp(&b.id));
    units
}

/// Scan story bundles for knowledge (only done stories).
fn scan_story_knowledge(board_dir: &Path) -> Result<Vec<Knowledge>> {
    let stories_dir = board_dir.join("stories");

    if !stories_dir.exists() {
        return Ok(Vec::new());
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
    let mut all = Vec::new();
    for bundle_path in entries {
        let readme_path = bundle_path.join("README.md");
        let reflect_path = bundle_path.join("REFLECT.md");

        if !readme_path.exists() || !reflect_path.exists() {
            continue;
        }

        let readme_content = match fs::read_to_string(&readme_path) {
            Ok(content) => content,
            Err(_) => continue,
        };

        if !readme_content.contains("status: done") {
            continue;
        }

        let reflect_content = match fs::read_to_string(&reflect_path) {
            Ok(content) => content,
            Err(_) => continue,
        };

        let scope = extract_scope_from_frontmatter(&readme_content);
        let source_story_id = bundle_path
            .file_name()
            .and_then(|name| name.to_str())
            .map(ToOwned::to_owned);

        let inline = parse_reflection_inline_knowledge(
            &reflect_content,
            &reflect_path,
            scope.clone(),
            source_story_id.clone(),
        );
        all.extend(dedupe_by_id(inline));
    }

    Ok(all)
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
            let created_at = content_created_at(&content);

            // Only extract from ## Synthesis section
            let section_content = extract_section(&content, "## Synthesis");
            let Some(section_content) = section_content else {
                return Vec::new();
            };
            parse_knowledge_with_metadata(
                &section_content,
                path,
                KnowledgeSourceType::Voyage,
                created_at,
            )
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

fn scan_catalog_knowledge(board_dir: &Path) -> Result<Vec<Knowledge>> {
    let knowledge_dir = knowledge_dir(board_dir);
    if !knowledge_dir.exists() {
        return Ok(Vec::new());
    }

    let files: Vec<_> = WalkDir::new(&knowledge_dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "md"))
        .map(|entry| entry.into_path())
        .collect();

    files
        .into_iter()
        .map(|path| load_knowledge_file(board_dir, &path))
        .collect()
}

fn merge_catalog_units(catalog: Vec<Knowledge>, discovered: Vec<Knowledge>) -> Vec<Knowledge> {
    let mut merged = HashMap::new();
    for unit in catalog {
        merged.insert(unit.id.clone(), unit);
    }
    for unit in discovered {
        merged.insert(unit.id.clone(), unit);
    }

    let mut units: Vec<_> = merged.into_values().collect();
    units.sort_by(|a, b| a.id.cmp(&b.id));
    units
}

fn scan_all_knowledge_sources(board_dir: &Path) -> Result<Vec<Knowledge>> {
    let story = scan_story_knowledge(board_dir)?;
    let (voyage, adhoc) = rayon::join(
        || scan_voyage_knowledge(board_dir),
        || scan_adhoc_knowledge(board_dir),
    );
    let catalog = scan_catalog_knowledge(board_dir)?;

    let mut discovered = Vec::with_capacity(story.len() + voyage.len() + adhoc.len());
    discovered.extend(story);
    discovered.extend(voyage);
    discovered.extend(adhoc);
    Ok(merge_catalog_units(catalog, discovered))
}

fn write_knowledge_catalog_files(board_dir: &Path, units: &[Knowledge]) -> Result<()> {
    let knowledge_dir = knowledge_dir(board_dir);
    fs::create_dir_all(&knowledge_dir)?;

    let active_ids: HashSet<_> = units.iter().map(|unit| unit.id.clone()).collect();
    for unit in units {
        let path = knowledge_file_path(board_dir, &unit.id);
        fs::write(&path, render_knowledge_markdown(board_dir, unit)?)
            .with_context(|| format!("Failed to write {}", path.display()))?;
    }

    for entry in fs::read_dir(&knowledge_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_none_or(|ext| ext != "md") {
            continue;
        }
        let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) else {
            continue;
        };
        if !active_ids.contains(stem) {
            fs::remove_file(&path)
                .with_context(|| format!("Failed to remove stale {}", path.display()))?;
        }
    }

    Ok(())
}

/// Scan all knowledge sources, enforce canonical IDs, and persist the catalog files.
pub fn sync_knowledge_catalog(board_dir: &Path) -> Result<KnowledgeCatalog> {
    let mut all = scan_all_knowledge_sources(board_dir)?;
    normalize_units(&mut all);

    let invalid_ids: Vec<String> = all
        .iter()
        .map(|unit| unit.id.clone())
        .filter(|id| !is_canonical_knowledge_id(id))
        .collect();
    if !invalid_ids.is_empty() {
        let mut sorted = invalid_ids;
        sorted.sort();
        sorted.dedup();
        bail!(
            "knowledge IDs must use canonical generated format (9-character base62). invalid IDs: {}",
            sorted.join(", ")
        );
    }

    let duplicates = duplicate_ids(&all);
    if !duplicates.is_empty() {
        bail!(
            "duplicate canonical knowledge IDs found across the board: {}",
            duplicates.join(", ")
        );
    }

    normalize_similarity_fields(&mut all);
    write_knowledge_catalog_files(board_dir, &all)?;
    Ok(KnowledgeCatalog {
        generated_at: Utc::now(),
        units: all,
    })
}

/// Scan all knowledge sources and return all knowledge units.
pub fn scan_all_knowledge(board_dir: &Path) -> Result<Vec<Knowledge>> {
    Ok(sync_knowledge_catalog(board_dir)?.units)
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

fn recency_score(unit: &Knowledge) -> f64 {
    let observed = unit.observed_at.or_else(|| {
        crate::infrastructure::story_id::extract_timestamp(&unit.id)
            .and_then(|secs| chrono::DateTime::<Utc>::from_timestamp(secs as i64, 0))
    });

    let Some(observed) = observed else {
        return 0.40;
    };

    let age_days = (Utc::now() - observed).num_days().max(0) as f64;
    (1.0 / (1.0 + (age_days / 30.0))).clamp(0.0, 1.0)
}

fn scope_match_score(unit: &Knowledge, epic: Option<&str>, scope: Option<&str>) -> f64 {
    if let Some(current_scope) = scope
        && unit.scope.as_deref() == Some(current_scope)
    {
        return 1.0;
    }

    if let Some(current_epic) = epic
        && let Some(unit_scope) = &unit.scope
        && unit_scope.starts_with(current_epic)
    {
        return 0.75;
    }

    0.0
}

/// Rank pending knowledge by scope match, confidence, and recency.
pub fn rank_relevant_knowledge(
    units: Vec<Knowledge>,
    epic: Option<&str>,
    scope: Option<&str>,
    limit: usize,
) -> Vec<RankedKnowledge> {
    let mut ranked: Vec<RankedKnowledge> = units
        .into_iter()
        .filter(|unit| unit.is_pending())
        .map(|unit| {
            let scope_match = scope_match_score(&unit, epic, scope);
            let recency = recency_score(&unit);
            let confidence = unit.confidence.clamp(0.0, 1.0);
            let score = 0.55 * scope_match + 0.30 * confidence + 0.15 * recency;

            RankedKnowledge {
                knowledge: unit,
                score,
                scope_match,
                recency,
            }
        })
        .filter(|ranked| ranked.scope_match > 0.0 || epic.is_none() && scope.is_none())
        .collect();

    ranked.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.knowledge.id.cmp(&b.knowledge.id))
    });
    ranked.truncate(limit);
    ranked
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

pub const NEAR_DUPLICATE_KNOWLEDGE_THRESHOLD: f64 = 0.95;

#[derive(Debug, Clone, PartialEq)]
pub struct KnowledgeSimilarityConflict {
    pub candidate_id: String,
    pub existing_id: String,
    pub similarity_score: f64,
}

fn similarity_tokens(unit: &Knowledge) -> HashSet<String> {
    format!("{} {}", unit.title, unit.insight)
        .split_whitespace()
        .map(|token| {
            token
                .trim_matches(|ch: char| !ch.is_ascii_alphanumeric())
                .to_ascii_lowercase()
        })
        .filter(|token| token.len() >= 3)
        .collect()
}

fn similarity_score_between(left: &Knowledge, right: &Knowledge) -> f64 {
    let left_tokens = similarity_tokens(left);
    let right_tokens = similarity_tokens(right);
    if left_tokens.is_empty() || right_tokens.is_empty() {
        return 0.0;
    }

    let intersection = left_tokens.intersection(&right_tokens).count() as f64;
    let union = left_tokens.union(&right_tokens).count() as f64;
    if union <= 0.0 {
        0.0
    } else {
        intersection / union
    }
}

pub fn detect_similarity_conflicts(
    candidates: &[Knowledge],
    existing: &[Knowledge],
    threshold: f64,
) -> Vec<KnowledgeSimilarityConflict> {
    let mut conflicts = Vec::new();

    for candidate in candidates {
        let mut best_existing: Option<(&Knowledge, f64)> = None;
        for unit in existing {
            if unit.id == candidate.id {
                continue;
            }

            let score = similarity_score_between(candidate, unit);
            if score < threshold {
                continue;
            }

            match best_existing {
                Some((_, best_score)) if best_score >= score => {}
                _ => best_existing = Some((unit, score)),
            }
        }

        if let Some((unit, score)) = best_existing
            && !candidate.linked_ids.iter().any(|linked| linked == &unit.id)
        {
            conflicts.push(KnowledgeSimilarityConflict {
                candidate_id: candidate.id.clone(),
                existing_id: unit.id.clone(),
                similarity_score: (score * 100.0).round() / 100.0,
            });
        }
    }

    conflicts
}

pub fn load_reflection_knowledge(board_dir: &Path, reflect_path: &Path) -> Result<Vec<Knowledge>> {
    let content = fs::read_to_string(reflect_path)
        .with_context(|| format!("Failed to read {}", reflect_path.display()))?;
    let inline = parse_knowledge_with_metadata(
        &content,
        reflect_path,
        KnowledgeSourceType::Story,
        content_created_at(&content),
    );
    let linked = load_reflection_linked_knowledge(board_dir, reflect_path, &content)?;
    Ok(dedupe_by_id(inline.into_iter().chain(linked).collect()))
}

pub fn parse_reflection_candidates(
    reflect_path: &Path,
    scope: Option<&str>,
    source_story_id: Option<&str>,
) -> Result<Vec<Knowledge>> {
    let content = fs::read_to_string(reflect_path)
        .with_context(|| format!("Failed to read {}", reflect_path.display()))?;
    Ok(parse_reflection_inline_knowledge(
        &content,
        reflect_path,
        scope.map(str::to_string),
        source_story_id.map(str::to_string),
    ))
}

fn relative_path(from_dir: &Path, to_path: &Path) -> PathBuf {
    let from_components: Vec<_> = from_dir.components().collect();
    let to_components: Vec<_> = to_path.components().collect();

    let mut shared = 0usize;
    while shared < from_components.len()
        && shared < to_components.len()
        && from_components[shared] == to_components[shared]
    {
        shared += 1;
    }

    let mut relative = PathBuf::new();
    for _ in shared..from_components.len() {
        relative.push("..");
    }
    for component in &to_components[shared..] {
        relative.push(component.as_os_str());
    }

    relative
}

fn render_reflection_knowledge_links(
    board_dir: &Path,
    reflect_path: &Path,
    units: &[Knowledge],
) -> String {
    let reflect_dir = reflect_path.parent().unwrap_or(board_dir);
    units
        .iter()
        .map(|unit| {
            let target = relative_path(reflect_dir, &knowledge_file_path(board_dir, &unit.id))
                .to_string_lossy()
                .replace('\\', "/");
            format!("- [{}]({}) {}", unit.id, target, unit.title)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn replace_markdown_section(content: &str, header: &str, section_body: &str) -> String {
    let header_with_newline = format!("{header}\n");
    if let Some(start_idx) = content.find(&header_with_newline) {
        let body_start = start_idx + header_with_newline.len();
        let remaining = &content[body_start..];
        let body_end = remaining
            .find("\n## ")
            .or_else(|| remaining.find("\n---"))
            .map(|offset| body_start + offset)
            .unwrap_or(content.len());

        let mut rewritten = String::new();
        rewritten.push_str(&content[..body_start]);
        if !section_body.is_empty() {
            rewritten.push('\n');
            rewritten.push_str(section_body);
            rewritten.push('\n');
        } else {
            rewritten.push('\n');
        }
        rewritten.push_str(&content[body_end..]);
        return rewritten;
    }

    let mut rewritten = content.trim_end().to_string();
    if !rewritten.is_empty() {
        rewritten.push_str("\n\n");
    }
    rewritten.push_str(header);
    rewritten.push('\n');
    if !section_body.is_empty() {
        rewritten.push('\n');
        rewritten.push_str(section_body);
        rewritten.push('\n');
    }
    rewritten
}

pub fn materialize_reflection_knowledge(
    board_dir: &Path,
    reflect_path: &Path,
    scope: Option<&str>,
    source_story_id: Option<&str>,
) -> Result<Vec<Knowledge>> {
    let content = fs::read_to_string(reflect_path)
        .with_context(|| format!("Failed to read {}", reflect_path.display()))?;
    let inline = parse_reflection_candidates(reflect_path, scope, source_story_id)?;

    if inline.is_empty() {
        return Ok(Vec::new());
    }

    fs::create_dir_all(knowledge_dir(board_dir))?;
    for unit in &inline {
        let path = knowledge_file_path(board_dir, &unit.id);
        fs::write(&path, render_knowledge_markdown(board_dir, unit)?)
            .with_context(|| format!("Failed to write {}", path.display()))?;
    }

    let mut linked = load_reflection_linked_knowledge(board_dir, reflect_path, &content)?;
    let mut all_links = dedupe_by_id(linked.drain(..).chain(inline.clone()).collect());
    all_links.sort_by(|a, b| a.id.cmp(&b.id));
    let rewritten = replace_markdown_section(
        &content,
        "## Knowledge",
        &render_reflection_knowledge_links(board_dir, reflect_path, &all_links),
    );
    fs::write(reflect_path, rewritten)
        .with_context(|| format!("Failed to update {}", reflect_path.display()))?;

    Ok(inline)
}

fn collect_knowledge_markdown_files(board_dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    let stories_dir = board_dir.join("stories");
    if stories_dir.exists() {
        for entry in WalkDir::new(&stories_dir)
            .into_iter()
            .filter_map(|entry| entry.ok())
        {
            if entry.file_name() == "REFLECT.md" {
                files.push(entry.path().to_path_buf());
            }
        }
    }

    let epics_dir = board_dir.join("epics");
    if epics_dir.exists() {
        for entry in WalkDir::new(&epics_dir)
            .into_iter()
            .filter_map(|entry| entry.ok())
        {
            if entry.file_name() == "KNOWLEDGE.md" {
                files.push(entry.path().to_path_buf());
            }
        }
    }

    if let Some(project_root) = board_dir.parent() {
        let adhoc_dir = project_root.join("knowledge");
        if adhoc_dir.exists() {
            for entry in WalkDir::new(&adhoc_dir)
                .into_iter()
                .filter_map(|entry| entry.ok())
            {
                if entry.path().extension().is_some_and(|ext| ext == "md") {
                    files.push(entry.path().to_path_buf());
                }
            }
        }
    }

    files.sort();
    files
}

fn rewrite_knowledge_headers_with_generated_ids(
    content: &str,
    used_ids: &mut HashSet<String>,
) -> (String, usize) {
    let mut rewritten = String::new();
    let mut regenerated = 0;
    let mut in_comment = false;

    for line in content.lines() {
        let mut current = line.to_string();
        let trimmed = line.trim_start();
        if trimmed.starts_with("<!--") {
            in_comment = true;
        }

        if !in_comment && let Some(caps) = KNOWLEDGE_HEADER_LINE_RE.captures(line) {
            let prefix = caps.get(1).map(|m| m.as_str()).unwrap_or("### ");
            let suffix = caps.get(3).map(|m| m.as_str()).unwrap_or(":");
            let mut generated = crate::infrastructure::story_id::generate_story_id();
            while used_ids.contains(&generated) {
                generated = crate::infrastructure::story_id::generate_story_id();
            }
            used_ids.insert(generated.clone());
            current = format!("{prefix}{generated}{suffix}");
            regenerated += 1;
        }

        rewritten.push_str(&current);
        rewritten.push('\n');

        if trimmed.ends_with("-->") {
            in_comment = false;
        }
    }

    if !content.ends_with('\n') {
        rewritten.pop();
    }

    (rewritten, regenerated)
}

/// One-time hard migration to regenerate canonical knowledge IDs and refresh catalog files.
pub fn migrate_legacy_knowledge_ids(board_dir: &Path) -> Result<KnowledgeMigrationReport> {
    let files = collect_knowledge_markdown_files(board_dir);
    let mut used_ids = HashSet::new();
    let mut files_updated = 0usize;
    let mut ids_regenerated = 0usize;

    for path in files {
        let original = std::fs::read_to_string(&path)?;
        let (rewritten, regenerated) =
            rewrite_knowledge_headers_with_generated_ids(&original, &mut used_ids);
        if regenerated > 0 {
            std::fs::write(&path, rewritten)?;
            files_updated += 1;
            ids_regenerated += regenerated;
        }
    }

    let catalog = sync_knowledge_catalog(board_dir)?;
    Ok(KnowledgeMigrationReport {
        files_updated,
        ids_regenerated,
        knowledge_entries: catalog.units.len(),
    })
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
| **Suggested Action** | Apply first action |

### L002: Second Knowledge

| Field | Value |
|-------|-------|
| **Category** | process |
| **Insight** | Second insight |
| **Suggested Action** | Apply second action |
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
| **Suggested Action** | Apply voyage action |
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
| **Suggested Action** | Add this assertion pattern |
"#;

        fs::write(bundle_dir.join("README.md"), readme_content).unwrap();
        fs::write(bundle_dir.join("REFLECT.md"), reflect_content).unwrap();

        let knowledge_list = scan_story_knowledge(temp.path()).unwrap();

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
| **Suggested Action** | Keep the synthesis contract stable |
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
                source_story_id: None,
                title: "Applied".to_string(),
                category: "code".to_string(),
                context: String::new(),
                insight: "Insight".to_string(),
                suggested_action: String::new(),
                applies_to: String::new(),
                applied: "file.md (2026-01-20)".to_string(),
                created_at: None,
                observed_at: None,
                score: 0.5,
                confidence: 0.8,
                linked_ids: Vec::new(),
                similar_to: None,
                similarity_score: None,
            },
            Knowledge {
                id: "L002".to_string(),
                source: Path::new("/b.md").to_path_buf(),
                source_type: KnowledgeSourceType::Story,
                scope: None,
                source_story_id: None,
                title: "Pending".to_string(),
                category: "code".to_string(),
                context: String::new(),
                insight: "Insight".to_string(),
                suggested_action: String::new(),
                applies_to: String::new(),
                applied: String::new(),
                created_at: None,
                observed_at: None,
                score: 0.5,
                confidence: 0.8,
                linked_ids: Vec::new(),
                similar_to: None,
                similarity_score: None,
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
                source_story_id: None,
                title: "Code".to_string(),
                category: "Code".to_string(),
                context: String::new(),
                insight: "Insight".to_string(),
                suggested_action: String::new(),
                applies_to: String::new(),
                applied: String::new(),
                created_at: None,
                observed_at: None,
                score: 0.5,
                confidence: 0.8,
                linked_ids: Vec::new(),
                similar_to: None,
                similarity_score: None,
            },
            Knowledge {
                id: "L002".to_string(),
                source: Path::new("/b.md").to_path_buf(),
                source_type: KnowledgeSourceType::Story,
                scope: None,
                source_story_id: None,
                title: "Process".to_string(),
                category: "process".to_string(),
                context: String::new(),
                insight: "Insight".to_string(),
                suggested_action: String::new(),
                applies_to: String::new(),
                applied: String::new(),
                created_at: None,
                observed_at: None,
                score: 0.5,
                confidence: 0.8,
                linked_ids: Vec::new(),
                similar_to: None,
                similarity_score: None,
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

    #[test]
    fn parse_knowledge_ignores_commented_examples() {
        let content = r#"
## Knowledge

<!--
### L001: Title
| Field | Value |
|-------|-------|
| **Insight** | Example insight |
| **Suggested Action** | Example action |
-->

### L002: Real Insight
| Field | Value |
|-------|-------|
| **Insight** | Real insight |
| **Suggested Action** | Real action |
"#;

        let knowledge_list = parse_knowledge_from_content(
            content,
            Path::new("/test.md"),
            KnowledgeSourceType::Story,
        );

        assert_eq!(knowledge_list.len(), 1);
        assert_eq!(knowledge_list[0].id, "L002");
    }

    #[test]
    fn validate_knowledge_reports_missing_required_fields() {
        let content = r#"
### 1AbCdE234: Implementation Insight
| Field | Value |
|-------|-------|
| **Insight** | |
| **Suggested Action** | |
"#;

        let issues = validate_knowledge_content(content);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].id, "1AbCdE234");
        assert!(issues[0].reason.contains("title"));
        assert!(issues[0].reason.contains("insight"));
        assert!(issues[0].reason.contains("suggested action"));
    }

    #[test]
    fn sync_knowledge_catalog_writes_individual_knowledge_files() {
        let temp = create_test_board();
        let bundle_dir = temp.path().join("stories/FEAT0002");
        fs::create_dir_all(&bundle_dir).unwrap();

        fs::write(
            bundle_dir.join("README.md"),
            "---\nid: FEAT0002\ntitle: Test Story\nscope: test-epic/01-test\nstatus: done\n---\n",
        )
        .unwrap();
        fs::write(
            bundle_dir.join("REFLECT.md"),
            r#"# Reflection

## Knowledge

### 1AbCdE234: Guard Duplicate Reflection Knowledge

| Field | Value |
|-------|-------|
| **Category** | process |
| **Context** | when promoting a story with reusable reflection notes |
| **Insight** | Only unique knowledge should be promoted into the catalog |
| **Suggested Action** | Materialize knowledge files before closing the story |
"#,
        )
        .unwrap();

        let catalog = sync_knowledge_catalog(temp.path()).unwrap();
        assert_eq!(catalog.units.len(), 1);

        let knowledge_path = knowledge_file_path(temp.path(), "1AbCdE234");
        let content = fs::read_to_string(knowledge_path).unwrap();
        assert!(content.contains("source_type: Story"));
        assert!(content.contains("source: stories/FEAT0002/REFLECT.md"));
        assert!(content.contains("### 1AbCdE234: Guard Duplicate Reflection Knowledge"));
    }

    #[test]
    fn load_reflection_knowledge_reads_linked_catalog_files() {
        let temp = create_test_board();
        let story_dir = temp.path().join("stories/FEAT0003");
        fs::create_dir_all(&story_dir).unwrap();
        let reflect_path = story_dir.join("REFLECT.md");

        let linked_knowledge = Knowledge {
            id: "1AbCdE235".to_string(),
            source: temp.path().join("stories/ORIGIN/REFLECT.md"),
            source_type: KnowledgeSourceType::Story,
            scope: Some("test-epic/01-test".to_string()),
            source_story_id: Some("ORIGIN".to_string()),
            title: "Prefer Linked Knowledge Files".to_string(),
            category: "process".to_string(),
            context: "when reflections should reference prior insight".to_string(),
            insight: "Reflection links should resolve through the knowledge catalog".to_string(),
            suggested_action: "Load linked knowledge files when rendering story context"
                .to_string(),
            applies_to: String::new(),
            applied: String::new(),
            created_at: None,
            observed_at: None,
            score: 0.7,
            confidence: 0.9,
            linked_ids: Vec::new(),
            similar_to: None,
            similarity_score: None,
        };
        fs::create_dir_all(knowledge_dir(temp.path())).unwrap();
        fs::write(
            knowledge_file_path(temp.path(), &linked_knowledge.id),
            render_knowledge_markdown(temp.path(), &linked_knowledge).unwrap(),
        )
        .unwrap();

        fs::write(
            &reflect_path,
            "# Reflection\n\n## Knowledge\n\n- [1AbCdE235](../../knowledge/1AbCdE235.md) Prefer Linked Knowledge Files\n",
        )
        .unwrap();

        let units = load_reflection_knowledge(temp.path(), &reflect_path).unwrap();
        assert_eq!(units.len(), 1);
        assert_eq!(units[0].id, "1AbCdE235");
        assert_eq!(units[0].title, "Prefer Linked Knowledge Files");
        assert_eq!(
            units[0].source,
            temp.path().join("stories/ORIGIN/REFLECT.md")
        );
    }

    #[test]
    fn materialize_reflection_knowledge_rewrites_reflect_to_catalog_links() {
        let temp = create_test_board();
        let story_dir = temp.path().join("stories/FEAT0004");
        fs::create_dir_all(&story_dir).unwrap();
        let reflect_path = story_dir.join("REFLECT.md");

        fs::write(
            &reflect_path,
            r#"# Reflection

## Knowledge

### 1AbCdE236: Canonical Reflection Output

| Field | Value |
|-------|-------|
| **Category** | testing |
| **Context** | when submit auto-completes a story |
| **Insight** | Reflection knowledge should move into dedicated files |
| **Suggested Action** | Replace inline tables with catalog links once accepted |

## Observations

The reflection stayed readable after linking.
"#,
        )
        .unwrap();

        let units = materialize_reflection_knowledge(
            temp.path(),
            &reflect_path,
            Some("test-epic/01-test"),
            Some("FEAT0004"),
        )
        .unwrap();

        assert_eq!(units.len(), 1);
        assert!(knowledge_file_path(temp.path(), "1AbCdE236").exists());

        let reflect = fs::read_to_string(&reflect_path).unwrap();
        assert!(reflect.contains("- [1AbCdE236](../../knowledge/1AbCdE236.md)"));
        assert!(!reflect.contains("### 1AbCdE236: Canonical Reflection Output"));
    }

    #[test]
    fn detect_similarity_conflicts_blocks_unlinked_near_duplicates() {
        let existing = Knowledge {
            id: "1AbCdE237".to_string(),
            source: Path::new("/existing.md").to_path_buf(),
            source_type: KnowledgeSourceType::Story,
            scope: None,
            source_story_id: None,
            title: "Avoid Duplicate Reflection Promotions".to_string(),
            category: "process".to_string(),
            context: String::new(),
            insight: "Promoting the same reflection knowledge twice creates noisy duplicates"
                .to_string(),
            suggested_action: "Link the existing knowledge instead of restating it".to_string(),
            applies_to: String::new(),
            applied: String::new(),
            created_at: None,
            observed_at: None,
            score: 0.5,
            confidence: 0.8,
            linked_ids: Vec::new(),
            similar_to: None,
            similarity_score: None,
        };
        let candidate = Knowledge {
            id: "1AbCdE238".to_string(),
            source: Path::new("/candidate.md").to_path_buf(),
            source_type: KnowledgeSourceType::Story,
            scope: None,
            source_story_id: None,
            title: "Avoid Duplicate Reflection Promotions".to_string(),
            category: "process".to_string(),
            context: String::new(),
            insight: "Promoting the same reflection knowledge twice creates noisy duplicates"
                .to_string(),
            suggested_action: "Link the existing knowledge instead of restating it".to_string(),
            applies_to: String::new(),
            applied: String::new(),
            created_at: None,
            observed_at: None,
            score: 0.5,
            confidence: 0.8,
            linked_ids: Vec::new(),
            similar_to: None,
            similarity_score: None,
        };

        let conflicts = detect_similarity_conflicts(
            &[candidate],
            &[existing],
            NEAR_DUPLICATE_KNOWLEDGE_THRESHOLD,
        );

        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0].existing_id, "1AbCdE237");
        assert_eq!(conflicts[0].similarity_score, 1.0);
    }
}
