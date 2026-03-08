//! Shared support for entity-scoped markdown file commands.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow};

use crate::cli::presentation::markdown;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct KnownDocument {
    pub stem: &'static str,
    pub filename: &'static str,
}

impl KnownDocument {
    pub(crate) const fn new(stem: &'static str, filename: &'static str) -> Self {
        Self { stem, filename }
    }
}

pub(crate) fn resolve_bundle_document_path(
    readme_path: &Path,
    entity_label: &str,
    entity_id: &str,
    requested: &str,
    documents: &[KnownDocument],
) -> Result<PathBuf> {
    let bundle_dir = readme_path.parent().unwrap_or(readme_path);
    let document = resolve_known_document(entity_label, requested, documents)?;
    let path = bundle_dir.join(document.filename);

    if path.exists() {
        return Ok(path);
    }

    let available = available_documents(bundle_dir, documents);
    let available_text = if available.is_empty() {
        "none".to_string()
    } else {
        available.join(", ")
    };

    Err(anyhow!(
        "{} '{}' does not have {} yet. Available files: {}",
        entity_label,
        entity_id,
        document.filename,
        available_text
    ))
}

pub(crate) fn print_markdown_file(path: &Path, raw: bool) -> Result<()> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read markdown file {}", path.display()))?;

    if raw {
        print_output(&content);
        return Ok(());
    }

    let width = crate::cli::presentation::terminal::get_terminal_width();
    let rendered = markdown::render_document(&content, width);
    print_output(&rendered);
    Ok(())
}

fn resolve_known_document<'a>(
    entity_label: &str,
    requested: &str,
    documents: &'a [KnownDocument],
) -> Result<&'a KnownDocument> {
    let normalized = normalize_doc_token(requested);
    if let Some(document) = documents
        .iter()
        .find(|document| normalize_doc_token(document.stem) == normalized)
    {
        return Ok(document);
    }

    let available: Vec<_> = documents.iter().map(|document| document.stem).collect();
    let suggestions = suggest_documents(&normalized, documents);

    if suggestions.is_empty() {
        Err(anyhow!(
            "Unknown {} document '{}'. Available files: {}",
            entity_label,
            requested,
            available.join(", ")
        ))
    } else {
        Err(anyhow!(
            "Unknown {} document '{}'. Did you mean: {}? Available files: {}",
            entity_label,
            requested,
            suggestions.join(", "),
            available.join(", ")
        ))
    }
}

fn available_documents(bundle_dir: &Path, documents: &[KnownDocument]) -> Vec<&'static str> {
    documents
        .iter()
        .filter(|document| bundle_dir.join(document.filename).exists())
        .map(|document| document.filename)
        .collect()
}

fn suggest_documents<'a>(requested: &str, documents: &'a [KnownDocument]) -> Vec<&'a str> {
    let mut scored: Vec<(usize, &str)> = documents
        .iter()
        .filter_map(|document| {
            let candidate = normalize_doc_token(document.stem);
            let distance = if candidate.starts_with(requested) || requested.starts_with(&candidate)
            {
                0
            } else if candidate.contains(requested) || requested.contains(&candidate) {
                1
            } else {
                levenshtein(requested, &candidate)
            };

            (distance <= 3).then_some((distance, document.stem))
        })
        .collect();

    scored.sort_by(|(left_distance, left_name), (right_distance, right_name)| {
        left_distance
            .cmp(right_distance)
            .then_with(|| left_name.cmp(right_name))
    });
    scored.truncate(3);
    scored.into_iter().map(|(_, stem)| stem).collect()
}

fn normalize_doc_token(value: &str) -> String {
    strip_md_suffix(value.trim())
        .to_ascii_uppercase()
        .replace(['-', ' '], "_")
}

fn strip_md_suffix(value: &str) -> &str {
    value
        .strip_suffix(".md")
        .or_else(|| value.strip_suffix(".MD"))
        .or_else(|| value.strip_suffix(".Md"))
        .or_else(|| value.strip_suffix(".mD"))
        .unwrap_or(value)
}

fn levenshtein(left: &str, right: &str) -> usize {
    let left_chars: Vec<_> = left.chars().collect();
    let right_chars: Vec<_> = right.chars().collect();
    let mut costs: Vec<usize> = (0..=right_chars.len()).collect();

    for (left_index, left_char) in left_chars.iter().enumerate() {
        let mut previous = costs[0];
        costs[0] = left_index + 1;

        for (right_index, right_char) in right_chars.iter().enumerate() {
            let current = costs[right_index + 1];
            let substitution = if left_char == right_char {
                previous
            } else {
                previous + 1
            };
            let insertion = costs[right_index] + 1;
            let deletion = current + 1;
            costs[right_index + 1] = substitution.min(insertion).min(deletion);
            previous = current;
        }
    }

    costs[right_chars.len()]
}

fn print_output(content: &str) {
    if content.ends_with('\n') {
        print!("{content}");
    } else {
        println!("{content}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOCS: &[KnownDocument] = &[
        KnownDocument::new("README", "README.md"),
        KnownDocument::new("PRESS_RELEASE", "PRESS_RELEASE.md"),
    ];

    #[test]
    fn resolve_known_document_accepts_case_insensitive_stem_with_extension() {
        let document = resolve_known_document("Epic", "readme.MD", DOCS).unwrap();
        assert_eq!(document.filename, "README.md");
    }

    #[test]
    fn resolve_known_document_suggests_typos() {
        let err = resolve_known_document("Epic", "reamde", DOCS)
            .unwrap_err()
            .to_string();

        assert!(err.contains("Did you mean: README"));
    }

    #[test]
    fn resolve_bundle_document_path_reports_available_files_when_missing() {
        let temp = tempfile::tempdir().unwrap();
        let readme_path = temp.path().join("README.md");
        fs::write(&readme_path, "# README").unwrap();

        let err =
            resolve_bundle_document_path(&readme_path, "Epic", "epic-1", "PRESS_RELEASE", DOCS)
                .unwrap_err()
                .to_string();

        assert!(err.contains("does not have PRESS_RELEASE.md yet"));
        assert!(err.contains("README.md"));
    }

    #[test]
    fn levenshtein_reports_close_distance_for_common_typo() {
        assert_eq!(levenshtein("README", "REAMDE"), 2);
    }
}
