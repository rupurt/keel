//! Shared markdown section extraction and lightweight excerpt parsing helpers.

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SectionExcerpt {
    pub paragraphs: Vec<String>,
    pub list_items: Vec<String>,
}

impl SectionExcerpt {
    pub fn is_empty(&self) -> bool {
        self.paragraphs.is_empty() && self.list_items.is_empty()
    }
}

pub fn extract_section(content: &str, heading: &str) -> Option<String> {
    extract_section_with_options(content, heading, false)
}

pub fn extract_section_with_options(
    content: &str,
    heading: &str,
    stop_at_thematic_break: bool,
) -> Option<String> {
    let mut in_section = false;
    let mut result = String::new();
    let heading_level = heading.chars().take_while(|ch| *ch == '#').count();

    for line in content.lines() {
        if line.starts_with(heading) {
            in_section = true;
            continue;
        }

        if !in_section {
            continue;
        }

        if stop_at_thematic_break && line.trim() == "---" {
            break;
        }

        if line.starts_with('#') {
            let level = line.chars().take_while(|ch| *ch == '#').count();
            if level <= heading_level {
                break;
            }
        }

        result.push_str(line);
        result.push('\n');
    }

    if result.trim().is_empty() {
        None
    } else {
        Some(result)
    }
}

pub fn extract_section_excerpt(section: &str) -> Option<SectionExcerpt> {
    let mut paragraphs = Vec::new();
    let mut list_items = Vec::new();
    let mut paragraph_lines = Vec::new();
    let mut list_item_lines = Vec::new();
    let mut in_list = false;

    for line in section.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            flush_paragraph(&mut paragraphs, &mut paragraph_lines);
            flush_list_item(&mut list_items, &mut list_item_lines);
            in_list = false;
            continue;
        }

        if trimmed.starts_with("<!--") {
            continue;
        }

        if let Some(item) = parse_list_item_start(trimmed) {
            flush_paragraph(&mut paragraphs, &mut paragraph_lines);
            flush_list_item(&mut list_items, &mut list_item_lines);
            list_item_lines.push(item.to_string());
            in_list = true;
            continue;
        }

        if in_list {
            list_item_lines.push(trimmed.to_string());
            continue;
        }

        paragraph_lines.push(trimmed.to_string());
    }

    flush_paragraph(&mut paragraphs, &mut paragraph_lines);
    flush_list_item(&mut list_items, &mut list_item_lines);

    let excerpt = SectionExcerpt {
        paragraphs,
        list_items,
    };

    (!excerpt.is_empty()).then_some(excerpt)
}

pub fn parse_markdown_list_items(section: &str) -> Vec<String> {
    extract_section_excerpt(section)
        .map(|excerpt| excerpt.list_items)
        .unwrap_or_default()
}

fn flush_paragraph(paragraphs: &mut Vec<String>, paragraph_lines: &mut Vec<String>) {
    if paragraph_lines.is_empty() {
        return;
    }

    paragraphs.push(paragraph_lines.join(" "));
    paragraph_lines.clear();
}

fn flush_list_item(list_items: &mut Vec<String>, list_item_lines: &mut Vec<String>) {
    if list_item_lines.is_empty() {
        return;
    }

    list_items.push(list_item_lines.join(" "));
    list_item_lines.clear();
}

fn parse_list_item_start(line: &str) -> Option<&str> {
    line.strip_prefix("- [ ] ")
        .map(str::trim)
        .or_else(|| line.strip_prefix("- [x] ").map(str::trim))
        .or_else(|| line.strip_prefix("- [X] ").map(str::trim))
        .or_else(|| line.strip_prefix("- ").map(str::trim))
        .or_else(|| parse_ordered_list_item(line))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_section_stops_at_same_or_higher_heading() {
        let content = r#"
# Title

## One

Alpha

### Detail

Still alpha

## Two

Beta
"#;

        let section = extract_section(content, "## One").unwrap();
        assert!(section.contains("Alpha"));
        assert!(section.contains("Still alpha"));
        assert!(!section.contains("Beta"));
    }

    #[test]
    fn extract_section_with_options_can_stop_at_thematic_break() {
        let content = r#"
## Synthesis

Alpha

---

## Next

Beta
"#;

        let section = extract_section_with_options(content, "## Synthesis", true).unwrap();
        assert!(section.contains("Alpha"));
        assert!(!section.contains("Next"));
    }

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
    fn parse_markdown_list_items_preserves_multiline_items() {
        let section = r#"
- first line
  second line
1. third line
   fourth line
"#;

        assert_eq!(
            parse_markdown_list_items(section),
            vec![
                "first line second line".to_string(),
                "third line fourth line".to_string(),
            ]
        );
    }

    #[test]
    fn extract_section_excerpt_keeps_paragraphs_and_list_items() {
        let section = r#"
Works in principle.

- First dependency
- Second dependency
"#;

        let excerpt = extract_section_excerpt(section).unwrap();
        assert_eq!(excerpt.paragraphs, vec!["Works in principle.".to_string()]);
        assert_eq!(
            excerpt.list_items,
            vec![
                "First dependency".to_string(),
                "Second dependency".to_string()
            ]
        );
    }
}
