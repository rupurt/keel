//! Frontmatter parser for markdown files
//!
//! Extracts YAML frontmatter from markdown and deserializes into typed structs.

use anyhow::{Result, anyhow};
use serde::de::DeserializeOwned;

/// Parse YAML frontmatter from markdown content
///
/// Returns a tuple of (parsed frontmatter, remaining body content).
/// Frontmatter must be delimited by `---` at the start and end.
pub fn parse_frontmatter<T: DeserializeOwned>(content: &str) -> Result<(T, &str)> {
    // Must start with ---
    let content = content.trim_start();
    if !content.starts_with("---") {
        return Err(anyhow!("Frontmatter must start with '---'"));
    }

    // Find the end of the opening delimiter line
    let after_open = &content[3..];
    let after_open = after_open.trim_start_matches([' ', '\t']);
    if !after_open.starts_with('\n') && !after_open.starts_with("\r\n") {
        return Err(anyhow!("Opening '---' must be on its own line"));
    }
    let yaml_start = content.len() - after_open.len() + 1; // Skip the newline

    // Find closing ---
    let rest = &content[yaml_start..];
    let closing_pos = rest
        .find("\n---")
        .or_else(|| rest.find("\r\n---"))
        .ok_or_else(|| anyhow!("Missing closing '---' delimiter"))?;

    let yaml_content = &rest[..closing_pos];
    let after_closing = &rest[closing_pos + 1..]; // Skip the newline before ---

    // Find where body starts (after the closing --- line)
    let body_start = after_closing
        .find('\n')
        .map(|i| i + 1)
        .unwrap_or(after_closing.len());
    let body = &after_closing[body_start..];

    // Parse YAML
    let frontmatter: T = serde_yaml::from_str(yaml_content)
        .map_err(|e| anyhow!("Failed to parse YAML frontmatter: {}", e))?;

    Ok((frontmatter, body))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryFrontmatter;

    #[test]
    fn parse_valid_story_frontmatter() {
        let content = r#"---
id: FEAT0001
title: Test story
type: feat
status: backlog
---

# Test story

This is the body content.
"#;

        let (fm, body): (StoryFrontmatter, &str) = parse_frontmatter(content).unwrap();

        assert_eq!(fm.id, "FEAT0001");
        assert_eq!(fm.title, "Test story");
        assert!(body.contains("# Test story"));
        assert!(body.contains("This is the body content."));
    }

    #[test]
    fn parse_frontmatter_with_optional_fields() {
        let content = r#"---
id: BUG0001
title: Fix crash
type: bug
status: in-progress
---
Body here
"#;

        let (fm, _): (StoryFrontmatter, &str) = parse_frontmatter(content).unwrap();

        assert_eq!(fm.id, "BUG0001");
        assert!(fm.scope.is_none());
    }

    #[test]
    fn parse_frontmatter_missing_opening_delimiter() {
        let content = r#"id: FEAT0001
title: Test
---
Body
"#;

        let result: Result<(StoryFrontmatter, &str)> = parse_frontmatter(content);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("must start with"));
    }

    #[test]
    fn parse_frontmatter_missing_closing_delimiter() {
        let content = r#"---
id: FEAT0001
title: Test
Body without closing delimiter
"#;

        let result: Result<(StoryFrontmatter, &str)> = parse_frontmatter(content);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("closing"));
    }

    #[test]
    fn parse_frontmatter_invalid_yaml() {
        let content = r#"---
id: FEAT0001
title: [unclosed bracket
---
Body
"#;

        let result: Result<(StoryFrontmatter, &str)> = parse_frontmatter(content);
        assert!(result.is_err());
    }

    #[test]
    fn parse_frontmatter_empty_body() {
        let content = r#"---
id: FEAT0001
title: Test
type: feat
status: backlog
---
"#;

        let (fm, body): (StoryFrontmatter, &str) = parse_frontmatter(content).unwrap();

        assert_eq!(fm.id, "FEAT0001");
        assert!(body.trim().is_empty());
    }

    #[test]
    fn parse_frontmatter_preserves_body_whitespace() {
        let content = "---\nid: FEAT0001\ntitle: Test\ntype: feat\nstatus: backlog\n---\n\n\nBody with leading newlines\n";

        let (_, body): (StoryFrontmatter, &str) = parse_frontmatter(content).unwrap();

        assert!(body.starts_with("\n\n"));
    }
}
