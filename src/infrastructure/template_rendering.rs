//! Shared template rendering service.
//!
//! Replaces `{{placeholder}}` tokens with concrete values.

/// Render a template by replacing `{{placeholder}}` patterns.
pub fn render(template: &str, replacements: &[(&str, &str)]) -> String {
    let mut content = template.to_string();
    for (placeholder, value) in replacements {
        content = content.replace(&format!("{{{{{}}}}}", placeholder), value);
    }
    content
}

#[cfg(test)]
mod tests {
    use super::render;

    #[test]
    fn render_replaces_placeholders() {
        let template = "Hello {{name}}, created at {{created_at}}";
        let result = render(
            template,
            &[("name", "World"), ("created_at", "2026-03-02T00:00:00")],
        );
        assert_eq!(result, "Hello World, created at 2026-03-02T00:00:00");
    }
}
