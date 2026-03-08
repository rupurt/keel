//! Shared template rendering service.
//!
//! Replaces `{{placeholder}}` tokens with concrete values.

use crate::infrastructure::frontmatter_mutation::{Mutation, apply};

/// Render a template by replacing `{{placeholder}}` patterns.
pub fn render(template: &str, replacements: &[(&str, &str)]) -> String {
    let mut content = template.to_string();
    for (placeholder, value) in replacements {
        content = content.replace(&format!("{{{{{}}}}}", placeholder), value);
    }
    content
}

/// Render a template, then apply a single batch of frontmatter mutations.
pub fn render_with_mutations(
    template: &str,
    replacements: &[(&str, &str)],
    mutations: &[Mutation],
) -> String {
    let rendered = render(template, replacements);
    if mutations.is_empty() {
        rendered
    } else {
        apply(&rendered, mutations)
    }
}

/// Render a template and strip an optional leading frontmatter block.
pub fn render_body(template: &str, replacements: &[(&str, &str)]) -> String {
    strip_optional_frontmatter(&render(template, replacements))
}

fn strip_optional_frontmatter(content: &str) -> String {
    let Some(without_prefix) = content.strip_prefix("---\n") else {
        return content.to_string();
    };
    let Some(frontmatter_end) = without_prefix.find("\n---\n") else {
        return content.to_string();
    };
    without_prefix[frontmatter_end + "\n---\n".len()..]
        .trim_start_matches('\n')
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::{render, render_body, render_with_mutations};
    use crate::infrastructure::frontmatter_mutation::Mutation;

    #[test]
    fn render_replaces_placeholders() {
        let template = "Hello {{name}}, created at {{created_at}}";
        let result = render(
            template,
            &[("name", "World"), ("created_at", "2026-03-02T00:00:00")],
        );
        assert_eq!(result, "Hello World, created at 2026-03-02T00:00:00");
    }

    #[test]
    fn render_with_mutations_updates_frontmatter_once() {
        let template = "\
---
id: TEST0001
title: {{title}}
status: draft
---

# {{title}}
";
        let result = render_with_mutations(
            template,
            &[("title", "Example")],
            &[
                Mutation::set("status", "planned"),
                Mutation::set("index", "3"),
            ],
        );

        assert!(result.contains("title: Example"));
        assert!(result.contains("status: planned"));
        assert!(result.contains("index: 3"));
        assert!(!result.contains("status: draft"));
    }

    #[test]
    fn render_body_strips_frontmatter_when_present() {
        let template = "\
---
created_at: {{created_at}}
---

# Body
";
        let result = render_body(template, &[("created_at", "2026-03-02T00:00:00")]);
        assert_eq!(result, "# Body\n");
    }
}
