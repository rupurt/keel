//! Knowledge scanner
//!
//! Scans stories, voyages, and ad-hoc files for knowledge.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use anyhow::{Result, bail};
use chrono::Utc;
use rayon::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

static KNOWLEDGE_FIELD_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\|\s*\*\*(\w+(?:\s+\w+)*)\*\*\s*\|\s*([^|]*)\|").unwrap());
static KNOWLEDGE_HEADER_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^###\s+([A-Za-z0-9]{9}|L\d+|ML\d+):\s*(.*)$").unwrap());
static KNOWLEDGE_HEADER_LINE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\s*###\s+)([A-Za-z0-9]{9}|L\d+|ML\d+)(:\s*.*)$").unwrap());
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

/// Canonical knowledge manifest persisted in `.keel/knowledge/manifest.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeManifest {
    pub schema_version: u32,
    pub generated_at: chrono::DateTime<Utc>,
    pub units: Vec<Knowledge>,
}

/// One-time migration result for canonicalizing knowledge IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KnowledgeMigrationReport {
    pub files_updated: usize,
    pub ids_regenerated: usize,
    pub manifest_entries: usize,
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

pub fn knowledge_manifest_path(board_dir: &Path) -> PathBuf {
    board_dir.join("knowledge").join("manifest.json")
}

pub fn load_knowledge_manifest(board_dir: &Path) -> Result<KnowledgeManifest> {
    let manifest_path = knowledge_manifest_path(board_dir);
    let content = std::fs::read_to_string(&manifest_path)?;
    Ok(serde_json::from_str(&content)?)
}

pub fn write_knowledge_manifest(
    board_dir: &Path,
    units: &[Knowledge],
) -> Result<KnowledgeManifest> {
    let manifest = KnowledgeManifest {
        schema_version: 1,
        generated_at: Utc::now(),
        units: units.to_vec(),
    };

    let manifest_path = knowledge_manifest_path(board_dir);
    if let Some(parent) = manifest_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(&manifest_path, serde_json::to_string_pretty(&manifest)?)?;
    Ok(manifest)
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
            let source_story_id = bundle_path
                .file_name()
                .and_then(|name| name.to_str())
                .map(ToOwned::to_owned);
            for knowledge in &mut knowledge_list {
                knowledge.scope = scope.clone();
                knowledge.source_story_id = source_story_id.clone();
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

fn scan_all_knowledge_sources(board_dir: &Path) -> Vec<Knowledge> {
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
    all
}

/// Scan all knowledge sources, enforce canonical IDs, and persist the manifest.
pub fn sync_knowledge_manifest(board_dir: &Path) -> Result<KnowledgeManifest> {
    let mut all = scan_all_knowledge_sources(board_dir);
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
    write_knowledge_manifest(board_dir, &all)
}

/// Scan all knowledge sources and return all knowledge units.
pub fn scan_all_knowledge(board_dir: &Path) -> Result<Vec<Knowledge>> {
    Ok(sync_knowledge_manifest(board_dir)?.units)
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

/// One-time hard migration to regenerate canonical knowledge IDs and write a manifest.
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

    let manifest = sync_knowledge_manifest(board_dir)?;
    Ok(KnowledgeMigrationReport {
        files_updated,
        ids_regenerated,
        manifest_entries: manifest.units.len(),
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
}
