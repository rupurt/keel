//! Declarative rendering primitives for `show` command outputs.
//!
//! Commands build a `ShowDocument` from reusable blocks, then render once.

use owo_colors::OwoColorize;

use crate::cli::style;
use crate::infrastructure::utils::visible_width;

#[derive(Debug, Clone, Default)]
pub struct ShowDocument {
    blocks: Vec<ShowBlock>,
}

impl ShowDocument {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_block(mut self, block: ShowBlock) -> Self {
        self.push_block(block);
        self
    }

    pub fn push_block(&mut self, block: ShowBlock) {
        self.blocks.push(block);
    }

    pub fn push_section(&mut self, section: ShowSection) {
        self.push_block(ShowBlock::Section(section));
    }

    pub fn push_key_values(&mut self, block: ShowKeyValues) {
        if !block.is_empty() {
            self.push_block(ShowBlock::KeyValues(block));
        }
    }

    pub fn push_header(&mut self, metadata: ShowKeyValues, rule_width: Option<usize>) {
        self.push_key_values(metadata);
        if let Some(width) = rule_width {
            self.push_rule(width);
        }
    }

    pub fn push_lines<I, S>(&mut self, lines: I)
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let lines: Vec<String> = lines.into_iter().map(Into::into).collect();
        if !lines.is_empty() {
            self.push_block(ShowBlock::Lines(lines));
        }
    }

    pub fn push_spacer(&mut self) {
        self.push_block(ShowBlock::Spacer);
    }

    pub fn push_sections_spaced<I>(&mut self, sections: I)
    where
        I: IntoIterator<Item = ShowSection>,
    {
        let mut first = true;
        for section in sections {
            if !first {
                self.push_spacer();
            }
            self.push_section(section);
            first = false;
        }
    }

    pub fn push_rule(&mut self, width: usize) {
        self.push_lines([crate::cli::style::rule(width, None)]);
    }

    pub fn render(&self) -> String {
        let mut lines = Vec::new();
        for block in &self.blocks {
            block.render_into(&mut lines);
        }

        while matches!(lines.last(), Some(last) if last.is_empty()) {
            lines.pop();
        }

        if lines.is_empty() {
            String::new()
        } else {
            let mut out = lines.join("\n");
            out.push('\n');
            out
        }
    }

    pub fn print(&self) {
        let rendered = self.render();
        if !rendered.is_empty() {
            print!("{rendered}");
        }
    }
}

#[derive(Debug, Clone)]
pub enum ShowBlock {
    KeyValues(ShowKeyValues),
    Section(ShowSection),
    Lines(Vec<String>),
    Spacer,
}

impl ShowBlock {
    fn render_into(&self, output: &mut Vec<String>) {
        match self {
            Self::KeyValues(block) => block.render_into(output),
            Self::Section(section) => section.render_into(output),
            Self::Lines(lines) => output.extend(lines.iter().cloned()),
            Self::Spacer => {
                if !output.is_empty() && !matches!(output.last(), Some(last) if last.is_empty()) {
                    output.push(String::new());
                }
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ShowSection {
    title: String,
    blocks: Vec<ShowBlock>,
}

impl ShowSection {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            blocks: Vec::new(),
        }
    }

    pub fn with_block(mut self, block: ShowBlock) -> Self {
        self.push_block(block);
        self
    }

    pub fn push_block(&mut self, block: ShowBlock) {
        self.blocks.push(block);
    }

    pub fn push_key_values(&mut self, block: ShowKeyValues) {
        if !block.is_empty() {
            self.push_block(ShowBlock::KeyValues(block));
        }
    }

    pub fn push_lines<I, S>(&mut self, lines: I)
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let lines: Vec<String> = lines.into_iter().map(Into::into).collect();
        if !lines.is_empty() {
            self.push_block(ShowBlock::Lines(lines));
        }
    }

    pub fn push_labeled_bullets<I, S, P>(
        &mut self,
        label: impl Into<String>,
        items: I,
        empty_placeholder: Option<P>,
    ) where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
        P: AsRef<str>,
    {
        let label = label.into();
        let items: Vec<String> = items
            .into_iter()
            .map(|item| item.as_ref().to_string())
            .collect();

        if items.is_empty() {
            if let Some(placeholder) = empty_placeholder {
                self.push_lines([format!(
                    "  {label} {}",
                    style::styled_inline_markdown(placeholder.as_ref())
                )]);
            }
            return;
        }

        self.push_lines([format!("  {label}")]);
        self.push_lines(
            items
                .into_iter()
                .map(|item| format!("    - {}", style::styled_inline_markdown(&item))),
        );
    }

    pub fn push_labeled_bullets_limited<I, S, P>(
        &mut self,
        label: impl Into<String>,
        items: I,
        max_items: usize,
        empty_placeholder: Option<P>,
    ) where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
        P: AsRef<str>,
    {
        let label = label.into();
        let items: Vec<String> = items
            .into_iter()
            .map(|item| item.as_ref().to_string())
            .collect();

        if items.is_empty() {
            if let Some(placeholder) = empty_placeholder {
                self.push_lines([format!(
                    "  {label} {}",
                    style::styled_inline_markdown(placeholder.as_ref())
                )]);
            }
            return;
        }

        let max_items = max_items.max(1);
        self.push_lines([format!("  {label}")]);
        self.push_lines(
            items
                .iter()
                .take(max_items)
                .map(|item| format!("    - {}", style::styled_inline_markdown(item))),
        );
        if items.len() > max_items {
            self.push_lines([format!(
                "    - ... {} more",
                items.len().saturating_sub(max_items)
            )]);
        }
    }

    pub fn push_labeled_text_block(&mut self, label: impl Into<String>, value: impl AsRef<str>) {
        let label = label.into();
        self.push_lines([format!("  {label}")]);

        let value_lines = text_block_lines(value.as_ref());

        if value_lines.is_empty() {
            return;
        }

        self.push_lines(value_lines);
    }

    pub fn push_labeled_text_block_limited(
        &mut self,
        label: impl Into<String>,
        value: impl AsRef<str>,
        max_paragraphs: usize,
    ) {
        let label = label.into();
        self.push_lines([format!("  {label}")]);

        let value = value.as_ref();
        let paragraphs = text_block_paragraphs(value);

        if paragraphs.is_empty() {
            return;
        }

        let max_paragraphs = max_paragraphs.max(1);
        let mut rendered = Vec::new();
        for (idx, paragraph) in paragraphs.iter().take(max_paragraphs).enumerate() {
            if idx > 0 {
                rendered.push(String::new());
            }
            rendered.extend(paragraph.iter().cloned());
        }
        if text_block_paragraph_count(value) > max_paragraphs {
            rendered.push("    ...".to_string());
        }

        self.push_lines(rendered);
    }

    fn render_into(&self, output: &mut Vec<String>) {
        output.push(self.title.bold().to_string());
        for block in &self.blocks {
            block.render_into(output);
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ShowKeyValues {
    rows: Vec<ShowFieldRow>,
    indent: usize,
    min_label_width: usize,
    bold_labels: bool,
}

impl ShowKeyValues {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_indent(mut self, indent: usize) -> Self {
        self.indent = indent;
        self
    }

    pub fn with_min_label_width(mut self, width: usize) -> Self {
        self.min_label_width = width;
        self
    }

    pub fn with_bold_labels(mut self, bold: bool) -> Self {
        self.bold_labels = bold;
        self
    }

    pub fn row(mut self, label: impl Into<String>, value: impl Into<String>) -> Self {
        self.push_row(label, value);
        self
    }

    pub fn row_optional<T>(mut self, label: impl Into<String>, value: Option<T>) -> Self
    where
        T: Into<String>,
    {
        self.push_optional_row(label, value);
        self
    }

    pub fn push_row(&mut self, label: impl Into<String>, value: impl Into<String>) {
        self.rows.push(ShowFieldRow {
            label: label.into(),
            value: value.into(),
        });
    }

    pub fn push_optional_row<T>(&mut self, label: impl Into<String>, value: Option<T>)
    where
        T: Into<String>,
    {
        if let Some(value) = value {
            self.push_row(label, value);
        }
    }

    pub fn push_standard_timestamps(
        &mut self,
        created: Option<String>,
        started: Option<String>,
        updated: Option<String>,
        completed: Option<String>,
    ) {
        self.push_optional_row("Created:", created);
        self.push_optional_row("Started:", started);
        self.push_optional_row("Updated:", updated);
        self.push_optional_row("Completed:", completed);
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    fn render_into(&self, output: &mut Vec<String>) {
        let width = self
            .rows
            .iter()
            .map(|row| visible_width(&row.label))
            .max()
            .unwrap_or(0)
            .max(self.min_label_width);

        let indent = " ".repeat(self.indent);
        for row in &self.rows {
            let label = if self.bold_labels {
                row.label.bold().to_string()
            } else {
                row.label.clone()
            };
            let pad = width.saturating_sub(visible_width(&row.label));
            output.push(format!(
                "{indent}{label}{} {}",
                " ".repeat(pad),
                style::styled_inline_markdown(&row.value)
            ));
        }
    }
}

fn text_block_paragraph_count(value: &str) -> usize {
    text_block_paragraphs(value).len()
}

fn text_block_paragraphs(value: &str) -> Vec<Vec<String>> {
    let mut paragraphs = Vec::new();
    let mut current = Vec::new();

    for line in value.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            if !current.is_empty() {
                paragraphs.push(current);
                current = Vec::new();
            }
            continue;
        }

        current.push(format!("    {}", style::styled_inline_markdown(trimmed)));
    }

    if !current.is_empty() {
        paragraphs.push(current);
    }

    paragraphs
}

fn text_block_lines(value: &str) -> Vec<String> {
    let mut lines = Vec::new();

    for (idx, paragraph) in text_block_paragraphs(value).into_iter().enumerate() {
        if idx > 0 {
            lines.push(String::new());
        }
        lines.extend(paragraph);
    }

    lines
}

#[derive(Debug, Clone)]
struct ShowFieldRow {
    label: String,
    value: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show_key_values_aligns_labels() {
        let block = ShowKeyValues::new()
            .with_min_label_width(8)
            .row("ID:", "A1")
            .row("Status:", "done");

        let mut lines = Vec::new();
        block.render_into(&mut lines);

        assert_eq!(lines[0], "ID:      A1");
        assert_eq!(lines[1], "Status:  done");
    }

    #[test]
    fn show_document_renders_sections_and_spacers() {
        let mut doc = ShowDocument::new();
        doc.push_key_values(ShowKeyValues::new().row("Title:", "Sample"));
        doc.push_spacer();
        doc.push_rule(20);
        doc.push_spacer();

        let section = ShowSection::new("Details").with_block(ShowBlock::Lines(vec![
            "  line one".to_string(),
            "  line two".to_string(),
        ]));
        doc.push_section(section);

        let rendered = doc.render();
        assert!(rendered.contains("Title: Sample"));
        assert!(rendered.contains("Details"));
        assert!(rendered.contains("  line one"));
        assert!(rendered.contains("  line two"));
        assert!(rendered.contains("\n\n"));
    }

    #[test]
    fn show_key_values_optional_rows_only_render_when_present() {
        let block = ShowKeyValues::new()
            .with_min_label_width(8)
            .row_optional("Scope:", None::<String>)
            .row_optional("Type:", Some("feat".to_string()));

        let mut lines = Vec::new();
        block.render_into(&mut lines);

        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0], "Type:    feat");
    }

    #[test]
    fn show_section_push_labeled_bullets_renders_placeholder() {
        let mut section = ShowSection::new("Summary");
        section.push_labeled_bullets(
            "Goals:",
            Vec::<String>::new(),
            Some("(not authored yet)".to_string()),
        );

        let mut lines = Vec::new();
        section.render_into(&mut lines);
        assert!(lines[0].contains("Summary"));
        assert_eq!(lines[1], "  Goals: (not authored yet)");
    }

    #[test]
    fn show_section_push_labeled_bullets_limited_truncates() {
        let mut section = ShowSection::new("Summary");
        section.push_labeled_bullets_limited("Items:", vec!["A", "B", "C"], 2, None::<String>);

        let mut lines = Vec::new();
        section.render_into(&mut lines);
        assert_eq!(lines[1], "  Items:");
        assert_eq!(lines[2], "    - A");
        assert_eq!(lines[3], "    - B");
        assert_eq!(lines[4], "    - ... 1 more");
    }

    #[test]
    fn show_section_push_labeled_text_block_places_value_under_label() {
        let mut section = ShowSection::new("Summary");
        section.push_labeled_text_block("Problem:", "Readable planning output");

        let mut lines = Vec::new();
        section.render_into(&mut lines);

        assert_eq!(lines[1], "  Problem:");
        assert_eq!(lines[2], "    Readable planning output");
    }

    #[test]
    fn show_section_push_labeled_text_block_limited_adds_ellipsis_on_new_line() {
        let mut section = ShowSection::new("Summary");
        section.push_labeled_text_block_limited(
            "Problem:",
            "One.\nStill one.\n\nTwo.\n\nThree.",
            1,
        );

        let mut lines = Vec::new();
        section.render_into(&mut lines);

        assert_eq!(lines[1], "  Problem:");
        assert_eq!(lines[2], "    One.");
        assert_eq!(lines[3], "    Still one.");
        assert_eq!(lines[4], "    ...");
    }

    #[test]
    fn show_section_push_labeled_text_block_preserves_paragraph_breaks() {
        let mut section = ShowSection::new("Summary");
        section.push_labeled_text_block("Problem:", "First paragraph.\n\nSecond paragraph.");

        let mut lines = Vec::new();
        section.render_into(&mut lines);

        assert_eq!(lines[1], "  Problem:");
        assert_eq!(lines[2], "    First paragraph.");
        assert_eq!(lines[3], "");
        assert_eq!(lines[4], "    Second paragraph.");
    }

    #[test]
    fn show_document_push_sections_spaced_inserts_single_blank_lines() {
        let mut doc = ShowDocument::new();
        doc.push_sections_spaced(vec![
            ShowSection::new("One").with_block(ShowBlock::Lines(vec!["  a".to_string()])),
            ShowSection::new("Two").with_block(ShowBlock::Lines(vec!["  b".to_string()])),
        ]);

        let rendered = doc.render();
        assert!(rendered.contains("\n  a\n\n"));
        assert!(rendered.contains("\n\n"));
        assert!(!rendered.contains("\n\n\n"));
    }

    #[test]
    fn show_key_values_push_standard_timestamps_renders_present_values_only() {
        let mut block = ShowKeyValues::new().with_min_label_width(9);
        block.push_standard_timestamps(
            Some("2026-03-05T10:00:00".to_string()),
            None,
            Some("2026-03-05T12:00:00".to_string()),
            None,
        );

        let mut lines = Vec::new();
        block.render_into(&mut lines);
        assert_eq!(lines.len(), 2);
        assert!(lines[0].starts_with("Created:"));
        assert!(lines[1].starts_with("Updated:"));
    }
}
