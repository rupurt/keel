//! Shared helpers for rewriting generated marker sections in authored documents.

use std::fs;
use std::path::Path;

use anyhow::{Result, anyhow};

use super::artifact_io::write_if_changed;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionPresence {
    Required,
    Optional,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SectionPatch<'a> {
    tag: &'a str,
    content: &'a str,
    presence: SectionPresence,
}

impl<'a> SectionPatch<'a> {
    pub fn required(tag: &'a str, content: &'a str) -> Self {
        Self {
            tag,
            content,
            presence: SectionPresence::Required,
        }
    }

    pub fn optional(tag: &'a str, content: &'a str) -> Self {
        Self {
            tag,
            content,
            presence: SectionPresence::Optional,
        }
    }
}

/// Rewrite one or more marked sections in a file.
pub fn rewrite_sections(path: &Path, patches: &[SectionPatch<'_>]) -> Result<()> {
    let original = fs::read_to_string(path)?;
    let updated = apply_patches(&original, patches)?;
    write_if_changed(path, &updated)?;
    Ok(())
}

/// Apply multiple section patches to a string in order.
pub fn apply_patches(original: &str, patches: &[SectionPatch<'_>]) -> Result<String> {
    let mut updated = original.to_string();

    for patch in patches {
        match replace_section(&updated, patch.tag, patch.content) {
            Some(next) => updated = next,
            None if patch.presence == SectionPresence::Optional => {}
            None => {
                let marker = format!("<!-- BEGIN {} -->", patch.tag);
                return Err(anyhow!("Missing {} marker", marker));
            }
        }
    }

    Ok(updated)
}

/// Update content between `<!-- BEGIN {tag} -->` and `<!-- END {tag} -->`.
pub fn update_section(original: &str, tag: &str, new_content: &str) -> Result<String> {
    replace_section(original, tag, new_content)
        .ok_or_else(|| anyhow!("Missing <!-- BEGIN {} --> marker", tag))
}

fn replace_section(original: &str, tag: &str, new_content: &str) -> Option<String> {
    let start_marker = format!("<!-- BEGIN {} -->", tag);
    let end_marker = format!("<!-- END {} -->", tag);

    let start_pos = original.find(&start_marker)?;
    let search_start = start_pos + start_marker.len();
    let end_pos = original[search_start..]
        .rfind(&end_marker)
        .map(|rel| search_start + rel)?;

    let mut result = original[..start_pos].to_string();
    result.push_str(&start_marker);
    result.push('\n');
    result.push_str(new_content.trim());
    result.push('\n');
    result.push_str(&end_marker);
    result.push_str(&original[end_pos + end_marker.len()..]);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn update_section_replaces_tagged_content() {
        let original = "Before\n<!-- BEGIN TEST -->\nOld\n<!-- END TEST -->\nAfter";

        let updated = update_section(original, "TEST", "New").unwrap();

        assert!(updated.contains("Before"));
        assert!(updated.contains("New"));
        assert!(updated.contains("After"));
        assert!(!updated.contains("Old"));
    }

    #[test]
    fn apply_patches_skips_optional_missing_sections() {
        let original = "Before\n<!-- BEGIN TEST -->\nOld\n<!-- END TEST -->\nAfter";

        let updated = apply_patches(
            original,
            &[
                SectionPatch::optional("MISSING", "Skipped"),
                SectionPatch::required("TEST", "New"),
            ],
        )
        .unwrap();

        assert!(updated.contains("New"));
        assert!(!updated.contains("Skipped"));
    }

    #[test]
    fn rewrite_sections_updates_file_in_place() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("README.md");
        fs::write(&path, "<!-- BEGIN TEST -->\nOld\n<!-- END TEST -->\n").unwrap();

        rewrite_sections(&path, &[SectionPatch::required("TEST", "New")]).unwrap();

        assert_eq!(
            fs::read_to_string(path).unwrap(),
            "<!-- BEGIN TEST -->\nNew\n<!-- END TEST -->\n"
        );
    }
}
