//! Shared template rendering service.
//!
//! Replaces `{{placeholder}}` tokens with concrete values.

use speccy::{Mutation, RenderOptions};

/// Render a template by replacing `{{placeholder}}` patterns.
pub fn render(template: &str, replacements: &[(&str, &str)]) -> String {
    speccy::render(template, replacements, RenderOptions::default())
        .expect("rendering without host hooks should be infallible")
}

/// Render a template, then apply a single batch of frontmatter mutations.
pub fn render_with_mutations(
    template: &str,
    replacements: &[(&str, &str)],
    mutations: &[Mutation],
) -> String {
    speccy::render(
        template,
        replacements,
        RenderOptions::new().with_mutations(mutations),
    )
    .expect("rendering without host hooks should be infallible")
}

/// Render a template and strip an optional leading frontmatter block.
pub fn render_body(template: &str, replacements: &[(&str, &str)]) -> String {
    speccy::render(
        template,
        replacements,
        RenderOptions::new().strip_frontmatter(),
    )
    .expect("rendering without host hooks should be infallible")
}

#[cfg(test)]
mod tests {
    use super::{render, render_body, render_with_mutations};
    use speccy::Mutation;

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
