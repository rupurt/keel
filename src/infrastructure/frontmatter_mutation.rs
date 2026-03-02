//! Shared frontmatter mutation service.
//!
//! Centralizes status/timestamp/scope field mutation so command handlers and
//! application services stop performing ad-hoc string replacements.

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mutation {
    Set { key: String, value: String },
    Remove { key: String },
}

impl Mutation {
    pub fn set(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self::Set {
            key: key.into(),
            value: value.into(),
        }
    }

    pub fn remove(key: impl Into<String>) -> Self {
        Self::Remove { key: key.into() }
    }

    fn key(&self) -> &str {
        match self {
            Self::Set { key, .. } | Self::Remove { key } => key,
        }
    }
}

/// Apply frontmatter mutations to markdown content.
///
/// - Existing keys are replaced/removed in-place.
/// - Missing `set` keys are inserted before the closing frontmatter delimiter.
pub fn apply(content: &str, mutations: &[Mutation]) -> String {
    let mut result = String::new();
    let mut in_frontmatter = false;
    let mut delimiter_count = 0;
    let mut handled = HashSet::new();

    for line in content.lines() {
        if line == "---" {
            delimiter_count += 1;
            in_frontmatter = delimiter_count == 1;

            // Inject missing keys before the closing frontmatter delimiter.
            if delimiter_count == 2 {
                for mutation in mutations {
                    if handled.contains(mutation.key()) {
                        continue;
                    }
                    if let Mutation::Set { key, value } = mutation {
                        result.push_str(&format!("{key}: {value}\n"));
                    }
                }
            }

            result.push_str(line);
            result.push('\n');
            continue;
        }

        if in_frontmatter
            && let Some(key) = frontmatter_key(line)
            && let Some(mutation) = lookup_mutation(mutations, key)
        {
            handled.insert(mutation.key().to_string());
            if let Mutation::Set { key, value } = mutation {
                result.push_str(&format!("{key}: {value}\n"));
            }
            continue;
        }

        result.push_str(line);
        result.push('\n');
    }

    result
}

fn lookup_mutation<'a>(mutations: &'a [Mutation], key: &str) -> Option<&'a Mutation> {
    // Last mutation wins when callers pass duplicate keys.
    mutations.iter().rev().find(|m| m.key() == key)
}

fn frontmatter_key(line: &str) -> Option<&str> {
    let (key, _) = line.split_once(':')?;
    let key = key.trim();
    if key.is_empty() { None } else { Some(key) }
}

#[cfg(test)]
mod tests {
    use super::{Mutation, apply};

    const SAMPLE: &str = r#"---
id: FEAT0001
title: Test Story
status: backlog
---

# Story
"#;

    #[test]
    fn set_replaces_existing_key() {
        let updated = apply(SAMPLE, &[Mutation::set("status", "in-progress")]);
        assert!(updated.contains("status: in-progress"));
        assert!(!updated.contains("status: backlog"));
    }

    #[test]
    fn set_inserts_missing_key_before_closing_delimiter() {
        let updated = apply(
            SAMPLE,
            &[
                Mutation::set("updated_at", "2026-03-02T00:00:00"),
                Mutation::set("scope", "epic/voyage"),
            ],
        );
        assert!(updated.contains("updated_at: 2026-03-02T00:00:00"));
        assert!(updated.contains("scope: epic/voyage"));
    }

    #[test]
    fn remove_deletes_existing_key() {
        let with_scope = r#"---
id: FEAT0001
status: backlog
scope: epic/voyage
---
Body
"#;
        let updated = apply(with_scope, &[Mutation::remove("scope")]);
        assert!(!updated.contains("scope: epic/voyage"));
    }

    #[test]
    fn last_duplicate_mutation_wins() {
        let updated = apply(
            SAMPLE,
            &[
                Mutation::set("status", "in-progress"),
                Mutation::set("status", "done"),
            ],
        );
        assert!(updated.contains("status: done"));
        assert!(!updated.contains("status: in-progress"));
    }
}
