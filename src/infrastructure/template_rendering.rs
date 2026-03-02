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
        let template = "Hello {{name}}, today is {{date}}";
        let result = render(template, &[("name", "World"), ("date", "Monday")]);
        assert_eq!(result, "Hello World, today is Monday");
    }
}
