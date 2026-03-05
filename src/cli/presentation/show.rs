//! Declarative rendering primitives for `show` command outputs.
//!
//! Commands build a `ShowDocument` from reusable blocks, then render once.

use owo_colors::OwoColorize;

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
            output.push(format!("{indent}{label}{} {}", " ".repeat(pad), row.value));
        }
    }
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
}
