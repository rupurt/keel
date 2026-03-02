//! Box UI component for terminal rendering

use owo_colors::OwoColorize;

/// A titled box with border and internal rules
pub struct BoxComponent {
    /// Title displayed in the top border
    pub title: String,
    /// Width of the box in characters
    pub width: usize,
    /// Lines of content inside the box
    pub lines: Vec<String>,
}

impl BoxComponent {
    /// Create a new box with the given title and width
    pub fn new(title: &str, width: usize) -> Self {
        Self {
            title: title.to_string(),
            width,
            lines: Vec::new(),
        }
    }

    /// Add a line of text to the box
    pub fn push_line(&mut self, line: String) {
        self.lines.push(line);
    }

    /// Add a horizontal rule inside the box
    pub fn push_rule(&mut self) {
        self.lines.push("---".to_string());
    }

    /// Get the internal content width
    pub fn width(&self) -> usize {
        self.width
    }

    /// Render the box as a vector of lines
    pub fn render(&self) -> Vec<String> {
        self.render_with_height(self.lines.len() + 2)
    }

    /// Render the box with a specific total height.
    ///
    /// If the box has fewer lines than height-2, it will be padded with empty content lines.
    /// Minimum height is 2 (top + bottom borders).
    pub fn render_with_height(&self, height: usize) -> Vec<String> {
        let mut output = Vec::new();
        let content_width = self.width - 2;
        let target_content_height = height.saturating_sub(2);

        // Top border with title
        let title_len = self.title.len();
        let border_len = if content_width > title_len + 4 {
            (content_width - title_len - 2) / 2
        } else {
            1
        };

        let top = format!(
            "┌{} {} {}┐",
            "─".repeat(border_len),
            self.title.bold(),
            "─".repeat(content_width - border_len - title_len - 2)
        );
        output.push(top);

        // Content lines
        for i in 0..target_content_height {
            if let Some(line) = self.lines.get(i) {
                if line == "---" {
                    output.push(format!("├{}┤", "─".repeat(content_width)));
                } else {
                    let padded =
                        crate::cli::presentation::flow::format::pad_to_width(line, content_width);
                    output.push(format!("│{}│", padded));
                }
            } else {
                // Padding line
                output.push(format!("│{}│", " ".repeat(content_width)));
            }
        }

        // Bottom border
        output.push(format!("└{}┘", "─".repeat(content_width)));

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_component_creates_with_fields() {
        let b = BoxComponent::new("Test", 40);
        assert_eq!(b.title, "Test");
        assert_eq!(b.width, 40);
    }

    #[test]
    fn box_render_produces_correct_structure() {
        let mut b = BoxComponent::new("Test", 20);
        b.push_line("Hello".to_string());
        let lines = b.render();

        assert_eq!(lines.len(), 3); // top, content, bottom
        assert!(lines[0].contains("Test"));
        assert!(lines[1].contains("Hello"));
    }
}
