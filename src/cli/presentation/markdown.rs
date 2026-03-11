//! Shared markdown document renderer for terminal output.

use owo_colors::OwoColorize;
use regex::Regex;
use std::sync::LazyLock;

use crate::cli::style;
use keel::infrastructure::utils::visible_width;

static MARKDOWN_LINK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").expect("valid markdown link regex"));
static MARKDOWN_IMAGE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"!\[([^\]]*)\]\(([^)]+)\)").expect("valid markdown image regex"));

/// Render a markdown document for the terminal, including syntax-colored YAML frontmatter.
pub fn render_document(content: &str, width: usize) -> String {
    let (frontmatter, body) = split_frontmatter_block(content);
    let mut parts = Vec::new();

    if let Some(frontmatter_block) = frontmatter {
        parts.push(render_code_block(frontmatter_block, "yaml"));
    }

    if !body.is_empty() {
        if frontmatter.is_some() && !body.starts_with('\n') && !body.starts_with("\r\n") {
            parts.push(String::new());
        }
        parts.push(render_body(body, width));
    }

    parts.join("\n")
}

fn render_body(body: &str, width: usize) -> String {
    let lines: Vec<&str> = body.lines().collect();
    let mut rendered = Vec::new();
    let mut index = 0;

    while index < lines.len() {
        let line = lines[index];
        let trimmed = line.trim();

        if trimmed.starts_with("```") {
            let (code_block, consumed) = collect_code_block(&lines[index..]);
            rendered.push(code_block);
            index += consumed;
            continue;
        }

        if is_table_start(&lines, index) {
            let (table_block, consumed) = collect_table_block(&lines[index..]);
            rendered.extend(table_block);
            index += consumed;
            continue;
        }

        if trimmed.is_empty() {
            rendered.push(String::new());
            index += 1;
            continue;
        }

        if is_thematic_break(trimmed) {
            rendered.push(style::rule(width, None));
            index += 1;
            continue;
        }

        if let Some((level, title)) = parse_heading(trimmed) {
            rendered.push(render_heading(level, title));
            index += 1;
            continue;
        }

        if let Some(blockquote) = render_blockquote(line) {
            rendered.push(blockquote);
            index += 1;
            continue;
        }

        if let Some(task_line) = render_task_list_item(line) {
            rendered.push(task_line);
            index += 1;
            continue;
        }

        if let Some(list_line) = render_list_item(line) {
            rendered.push(list_line);
            index += 1;
            continue;
        }

        rendered.push(render_inline(line));
        index += 1;
    }

    rendered.join("\n")
}

fn split_frontmatter_block(content: &str) -> (Option<&str>, &str) {
    if !(content.starts_with("---\n") || content.starts_with("---\r\n")) {
        return (None, content);
    }

    let after_open = if content.starts_with("---\r\n") { 5 } else { 4 };
    let remainder = &content[after_open..];
    let mut offset = after_open;

    for line in remainder.split_inclusive('\n') {
        offset += line.len();
        if line.trim_end_matches(['\r', '\n']) == "---" {
            return (Some(&content[..offset]), &content[offset..]);
        }
    }

    if remainder.trim_end_matches(['\r', '\n']) == "---" {
        return (Some(content), "");
    }

    (None, content)
}

fn collect_code_block(lines: &[&str]) -> (String, usize) {
    let opening = lines[0].trim();
    let lang = opening.trim_start_matches('`').trim();
    let mut consumed = 1;
    let mut code = String::new();

    while consumed < lines.len() {
        let line = lines[consumed];
        if line.trim().starts_with("```") {
            consumed += 1;
            break;
        }
        code.push_str(line);
        code.push('\n');
        consumed += 1;
    }

    (render_code_block(&code, lang), consumed)
}

fn render_code_block(code: &str, lang: &str) -> String {
    if let Some(highlighted) = style::highlight_code_block(code, lang) {
        return highlighted.trim_end_matches('\n').to_string();
    }

    code.lines()
        .map(|line| format!("{}", line.dimmed()))
        .collect::<Vec<_>>()
        .join("\n")
}

fn is_table_start(lines: &[&str], index: usize) -> bool {
    index + 1 < lines.len()
        && looks_like_table_row(lines[index])
        && is_table_separator_line(lines[index + 1])
}

fn collect_table_block(lines: &[&str]) -> (Vec<String>, usize) {
    let mut table_lines = Vec::new();
    let mut consumed = 0;

    while consumed < lines.len() {
        let line = lines[consumed];
        if line.trim().is_empty() || !looks_like_table_row(line) {
            break;
        }
        table_lines.push(line);
        consumed += 1;
    }

    (render_table(&table_lines), consumed)
}

fn render_table(lines: &[&str]) -> Vec<String> {
    if lines.len() < 2 {
        return lines.iter().map(|line| render_inline(line)).collect();
    }

    let header = split_pipe_row(lines[0]);
    let data_rows: Vec<Vec<String>> = lines
        .iter()
        .skip(2)
        .filter(|line| !is_table_separator_line(line))
        .map(|line| split_pipe_row(line))
        .collect();

    let column_count = header
        .len()
        .max(data_rows.iter().map(Vec::len).max().unwrap_or(0));
    let mut widths = vec![0; column_count];

    for row in std::iter::once(&header).chain(data_rows.iter()) {
        for (index, cell) in row.iter().enumerate() {
            let width = visible_width(&render_inline(cell));
            widths[index] = widths[index].max(width);
        }
    }

    let mut rendered = vec![
        render_table_row(&header, &widths, true),
        render_table_separator(&widths),
    ];
    rendered.extend(
        data_rows
            .iter()
            .map(|row| render_table_row(row, &widths, false)),
    );
    rendered
}

fn render_table_row(cells: &[String], widths: &[usize], header: bool) -> String {
    let mut row = String::new();
    row.push_str(&format!("{}", "|".dimmed()));

    for (index, width) in widths.iter().enumerate() {
        let rendered = cells
            .get(index)
            .map(|cell| render_inline(cell))
            .unwrap_or_default();
        let padded_width = width.saturating_sub(visible_width(&rendered));
        let content = if header {
            format!("{}", rendered.bold())
        } else {
            rendered
        };

        row.push(' ');
        row.push_str(&content);
        row.push_str(&" ".repeat(padded_width + 1));
        row.push_str(&format!("{}", "|".dimmed()));
    }

    row
}

fn render_table_separator(widths: &[usize]) -> String {
    let mut row = String::new();
    row.push_str(&format!("{}", "|".dimmed()));
    for width in widths {
        row.push_str(&format!(
            " {} {}",
            "-".repeat(*width).dimmed(),
            "|".dimmed()
        ));
    }
    row
}

fn split_pipe_row(line: &str) -> Vec<String> {
    line.trim()
        .trim_matches('|')
        .split('|')
        .map(|cell| cell.trim().to_string())
        .collect()
}

fn looks_like_table_row(line: &str) -> bool {
    let trimmed = line.trim();
    !trimmed.is_empty() && trimmed.contains('|')
}

fn is_table_separator_line(line: &str) -> bool {
    let cells = split_pipe_row(line);
    !cells.is_empty()
        && cells.iter().all(|cell| {
            let trimmed = cell.trim();
            !trimmed.is_empty() && trimmed.chars().all(|ch| matches!(ch, '-' | ':' | ' '))
        })
}

fn is_thematic_break(trimmed: &str) -> bool {
    let compact: String = trimmed.chars().filter(|ch| !ch.is_whitespace()).collect();
    compact.len() >= 3 && compact.chars().all(|ch| matches!(ch, '-' | '*' | '_'))
}

fn parse_heading(trimmed: &str) -> Option<(usize, &str)> {
    let level = trimmed.chars().take_while(|ch| *ch == '#').count();
    if !(1..=6).contains(&level) {
        return None;
    }

    let title = trimmed[level..]
        .trim_start()
        .trim_end_matches('#')
        .trim_end();
    if title.is_empty() {
        None
    } else {
        Some((level, title))
    }
}

fn render_heading(level: usize, title: &str) -> String {
    let rendered = render_inline(title);
    match level {
        1 | 2 => format!("{}", rendered.bold()),
        _ => rendered,
    }
}

fn render_blockquote(line: &str) -> Option<String> {
    let trimmed = line.trim_start();
    if !trimmed.starts_with('>') {
        return None;
    }

    let depth = trimmed.chars().take_while(|ch| *ch == '>').count();
    let content = trimmed[depth..].trim_start();
    let prefix = format!("{}", "│ ".repeat(depth).dimmed());
    Some(format!("{prefix}{}", render_inline(content)))
}

fn render_task_list_item(line: &str) -> Option<String> {
    let indent = " ".repeat(line.len() - line.trim_start().len());
    let trimmed = line.trim_start();
    let bullet = trimmed.chars().next()?;
    if !matches!(bullet, '-' | '*' | '+') || !trimmed[1..].starts_with(" [") {
        return None;
    }

    let rest = trimmed[2..].trim_start();
    let (checked, content) = if let Some(content) = rest.strip_prefix("[x] ") {
        (true, content)
    } else if let Some(content) = rest.strip_prefix("[X] ") {
        (true, content)
    } else if let Some(content) = rest.strip_prefix("[ ] ") {
        (false, content)
    } else {
        return None;
    };

    let checkbox = if checked {
        format!("{}", "✓".green())
    } else {
        format!("{}", "○".red())
    };

    Some(format!("{indent}{checkbox} {}", render_inline(content)))
}

fn render_list_item(line: &str) -> Option<String> {
    let indent = " ".repeat(line.len() - line.trim_start().len());
    let trimmed = line.trim_start();

    if let Some(content) = trimmed
        .strip_prefix("- ")
        .or_else(|| trimmed.strip_prefix("* "))
        .or_else(|| trimmed.strip_prefix("+ "))
    {
        return Some(format!("{indent}• {}", render_inline(content)));
    }

    let digit_count = trimmed.chars().take_while(|ch| ch.is_ascii_digit()).count();
    if digit_count > 0 && trimmed[digit_count..].starts_with(". ") {
        let marker = &trimmed[..digit_count + 1];
        let content = &trimmed[digit_count + 2..];
        return Some(format!(
            "{indent}{} {}",
            marker.cyan(),
            render_inline(content)
        ));
    }

    None
}

fn render_inline(text: &str) -> String {
    style::styled_inline_markdown(&normalize_links(text))
}

fn normalize_links(text: &str) -> String {
    let with_images = MARKDOWN_IMAGE_RE
        .replace_all(text, |captures: &regex::Captures<'_>| {
            if captures[1].is_empty() {
                format!("[image: {}]", &captures[2])
            } else {
                format!("{} [image: {}]", &captures[1], &captures[2])
            }
        })
        .into_owned();

    MARKDOWN_LINK_RE
        .replace_all(&with_images, |captures: &regex::Captures<'_>| {
            format!("{} ({})", &captures[1], &captures[2])
        })
        .into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    fn strip_ansi(value: &str) -> String {
        Regex::new(r"\x1b\[[0-9;]*[a-zA-Z]")
            .unwrap()
            .replace_all(value, "")
            .into_owned()
    }

    #[test]
    fn render_document_highlights_frontmatter_and_body() {
        let rendered = render_document(
            "---\nid: FEAT0001\ntitle: Test Story\n---\n# Heading\n\nBody text.",
            80,
        );
        let plain = strip_ansi(&rendered);

        assert!(plain.contains("id: FEAT0001"));
        assert!(plain.contains("Heading"));
        assert!(plain.contains("Body text."));
        assert!(rendered.contains("\u{1b}["));
    }

    #[test]
    fn render_document_formats_tables() {
        let rendered = render_document(
            "| Name | Count |\n| --- | --- |\n| Alpha | 1 |\n| Beta | 22 |",
            80,
        );

        assert!(rendered.contains("Alpha"));
        assert!(rendered.contains("Beta"));
        assert!(rendered.contains("|"));
    }

    #[test]
    fn render_document_formats_task_lists_and_code_fences() {
        let rendered = render_document("- [x] Done\n- [ ] Todo\n\n```rust\nfn main() {}\n```", 80);
        let plain = strip_ansi(&rendered);

        assert!(plain.contains('✓'));
        assert!(plain.contains('○'));
        assert!(plain.contains("fn main() {}"));
        assert!(!plain.contains("```"));
    }

    #[test]
    fn render_document_formats_links() {
        let rendered = render_document("[README](README.md)", 80);
        assert!(rendered.contains("README (README.md)"));
    }
}
