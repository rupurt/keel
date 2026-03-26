use std::collections::{HashMap, HashSet};

/// A generic markdown frontmatter mutation.
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
/// - Existing keys are replaced or removed in-place.
/// - Missing `set` keys are inserted before the closing frontmatter delimiter.
pub fn apply_frontmatter_mutations(content: &str, mutations: &[Mutation]) -> String {
    let mut result = String::new();
    let mut in_frontmatter = false;
    let mut delimiter_count = 0;
    let mut handled = HashSet::new();
    let mut handled_subs = HashSet::new();

    let mut nested_mutations: HashMap<String, Vec<(String, String)>> = HashMap::new();

    for mutation in mutations {
        if let Mutation::Set { key, value } = mutation
            && let Some((section, subkey)) = key.split_once('.')
        {
            nested_mutations
                .entry(section.to_string())
                .or_default()
                .push((subkey.to_string(), value.clone()));
        }
    }

    let mut current_section: Option<String> = None;

    for line in content.lines() {
        if line == "---" {
            delimiter_count += 1;
            in_frontmatter = delimiter_count == 1;

            if delimiter_count == 2 {
                if let Some(section) = &current_section
                    && let Some(subs) = nested_mutations.get(section)
                {
                    for (subkey, value) in subs {
                        let full_key = format!("{}.{}", section, subkey);
                        if !handled_subs.contains(&full_key) {
                            result.push_str(&format!("  {subkey}: {value}\n"));
                            handled_subs.insert(full_key);
                        }
                    }
                }

                for mutation in mutations {
                    if let Mutation::Set { key, value } = mutation
                        && !key.contains('.')
                        && !handled.contains(key)
                    {
                        result.push_str(&format!("{key}: {value}\n"));
                    }
                }

                for (section, subs) in &nested_mutations {
                    if !handled.contains(section) {
                        result.push_str(&format!("{section}:\n"));
                        for (subkey, value) in subs {
                            result.push_str(&format!("  {subkey}: {value}\n"));
                        }
                    }
                }
            }

            result.push_str(line);
            result.push('\n');
            continue;
        }

        if in_frontmatter {
            if let Some(key) = frontmatter_key(line) {
                if let Some(prev_section) = &current_section
                    && let Some(subs) = nested_mutations.get(prev_section)
                {
                    for (subkey, value) in subs {
                        let full_key = format!("{}.{}", prev_section, subkey);
                        if !handled_subs.contains(&full_key) {
                            result.push_str(&format!("  {subkey}: {value}\n"));
                            handled_subs.insert(full_key);
                        }
                    }
                }

                current_section = Some(key.to_string());

                if let Some(mutation) = lookup_mutation(mutations, key) {
                    handled.insert(key.to_string());
                    if let Mutation::Set { key, value } = mutation {
                        result.push_str(&format!("{key}: {value}\n"));
                    }
                    continue;
                }
            } else if line.starts_with("  ")
                && let Some(section) = &current_section
                && let Some(subs) = nested_mutations.get(section)
            {
                let trimmed = line.trim();
                if let Some((subkey, _)) = trimmed.split_once(':') {
                    let subkey = subkey.trim();
                    if let Some((_, value)) = subs.iter().find(|(key, _)| key == subkey) {
                        let full_key = format!("{}.{}", section, subkey);
                        result.push_str(&format!("  {subkey}: {value}\n"));
                        handled_subs.insert(full_key);
                        handled.insert(section.clone());
                        continue;
                    }
                }
            }
        }

        result.push_str(line);
        result.push('\n');
    }

    result
}

pub(crate) fn strip_optional_frontmatter(content: &str) -> String {
    let Some(without_prefix) = content.strip_prefix("---\n") else {
        return content.to_string();
    };
    let Some(frontmatter_end) = without_prefix.find("\n---\n") else {
        return content.to_string();
    };
    without_prefix[frontmatter_end + "\n---\n".len()..]
        .trim_start_matches('\n')
        .to_string()
}

fn lookup_mutation<'a>(mutations: &'a [Mutation], key: &str) -> Option<&'a Mutation> {
    mutations
        .iter()
        .rev()
        .find(|mutation| mutation.key() == key)
}

fn frontmatter_key(line: &str) -> Option<&str> {
    let (key, _) = line.split_once(':')?;
    let key = key.trim();
    if key.is_empty() { None } else { Some(key) }
}

#[cfg(test)]
mod tests {
    use super::{Mutation, apply_frontmatter_mutations};

    const SAMPLE: &str = r#"---
id: FEAT0001
title: Test Story
status: backlog
---

# Story
"#;

    #[test]
    fn set_replaces_existing_key() {
        let updated =
            apply_frontmatter_mutations(SAMPLE, &[Mutation::set("status", "in-progress")]);
        assert!(updated.contains("status: in-progress"));
        assert!(!updated.contains("status: backlog"));
    }

    #[test]
    fn set_inserts_missing_key_before_closing_delimiter() {
        let updated = apply_frontmatter_mutations(
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
        let updated = apply_frontmatter_mutations(with_scope, &[Mutation::remove("scope")]);
        assert!(!updated.contains("scope: epic/voyage"));
    }

    #[test]
    fn last_duplicate_mutation_wins() {
        let updated = apply_frontmatter_mutations(
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
