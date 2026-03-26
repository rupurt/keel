//! Reusable fixed-width scene primitives for terminal surfaces.
//!
//! The crate currently pilots the `keel flow --scene` extraction by owning
//! visible-width measurement, width-safe line assembly, framed rows, and
//! optional ANSI styling without depending on Keel internals.

use owo_colors::OwoColorize;
use regex::Regex;
use std::sync::LazyLock;

static ANSI_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\x1b\[[0-9;]*[a-zA-Z]").unwrap());

/// Returns the visible terminal width of a string after ANSI escape stripping.
///
/// The policy is intentionally single-cell: each non-ANSI char counts as one
/// column until later voyages add wider-glyph support explicitly.
pub fn visible_width(text: &str) -> usize {
    ANSI_RE.replace_all(text, "").chars().count()
}

/// Fixed-width line builder that measures visible columns instead of raw bytes.
#[derive(Debug, Clone)]
pub struct SceneLine {
    target_width: usize,
    content: String,
    visible_width: usize,
}

impl SceneLine {
    pub fn new(target_width: usize) -> Self {
        Self {
            target_width,
            content: String::new(),
            visible_width: 0,
        }
    }

    pub fn push(&mut self, text: impl AsRef<str>) -> &mut Self {
        let text = text.as_ref();
        self.visible_width += visible_width(text);
        self.content.push_str(text);
        self
    }

    pub fn pad_to(&mut self, column: usize) -> &mut Self {
        let padding = column.saturating_sub(self.visible_width);
        if padding > 0 {
            self.content.push_str(&" ".repeat(padding));
            self.visible_width += padding;
        }
        self
    }

    pub fn finish(mut self) -> String {
        self.pad_to(self.target_width);
        self.content
    }
}

/// Framed fixed-width scene rows with ANSI-aware content padding.
#[derive(Debug, Clone, Copy)]
pub struct SceneFrame<'a> {
    indent: &'a str,
    left_border: &'a str,
    right_border: &'a str,
    inner_width: usize,
}

impl<'a> SceneFrame<'a> {
    pub fn new(
        indent: &'a str,
        left_border: &'a str,
        right_border: &'a str,
        inner_width: usize,
    ) -> Self {
        Self {
            indent,
            left_border,
            right_border,
            inner_width,
        }
    }

    pub fn row<F>(&self, build: F) -> String
    where
        F: FnOnce(&mut SceneLine),
    {
        let mut line = SceneLine::new(self.inner_width);
        build(&mut line);
        format!(
            "{}{}{}{}",
            self.indent,
            self.left_border,
            line.finish(),
            self.right_border
        )
    }

    pub fn empty_row(&self) -> String {
        self.row(|_| {})
    }
}

/// Simple scene palette that can disable ANSI styling entirely.
#[derive(Debug, Clone, Copy)]
pub struct ScenePalette {
    use_color: bool,
}

impl ScenePalette {
    pub fn new(use_color: bool) -> Self {
        Self { use_color }
    }

    pub fn dim(&self, text: impl Into<String>) -> String {
        self.paint(text, |value| value.dimmed().to_string())
    }

    pub fn green(&self, text: impl Into<String>) -> String {
        self.paint(text, |value| value.green().to_string())
    }

    pub fn red_bold(&self, text: impl Into<String>) -> String {
        self.paint(text, |value| value.red().bold().to_string())
    }

    pub fn yellow_bold(&self, text: impl Into<String>) -> String {
        self.paint(text, |value| value.yellow().bold().to_string())
    }

    pub fn yellow_dim(&self, text: impl Into<String>) -> String {
        self.paint(text, |value| value.yellow().dimmed().to_string())
    }

    fn paint(&self, text: impl Into<String>, style: impl FnOnce(String) -> String) -> String {
        let text = text.into();
        if self.use_color { style(text) } else { text }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visible_width_ignores_ansi_sequences() {
        assert_eq!(visible_width("\x1b[32mAB\x1b[0m"), 2);
        assert_eq!(visible_width("plain"), 5);
    }

    #[test]
    fn scene_line_pads_to_visible_width_with_ansi_content() {
        let mut line = SceneLine::new(8);
        line.push("AB".green().to_string());
        line.push("CD");

        let rendered = line.finish();
        assert_eq!(visible_width(&rendered), 8);
        assert!(rendered.ends_with("    "));
    }

    #[test]
    fn scene_frame_wraps_rows_to_exact_visible_width() {
        let frame = SceneFrame::new("    ", "│", "│", 10);
        let rendered = frame.row(|line| {
            line.pad_to(2).push("X");
        });

        assert_eq!(visible_width(&rendered), 16);
        assert!(rendered.starts_with("    │"));
        assert!(rendered.ends_with('│'));
    }

    #[test]
    fn scene_palette_disables_color_when_requested() {
        let palette = ScenePalette::new(false);
        assert_eq!(palette.green("█"), "█");
        assert_eq!(palette.yellow_bold("line"), "line");
    }
}
