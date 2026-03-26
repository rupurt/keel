use std::collections::HashMap;

use anyhow::Result;

use crate::catalog::TemplateCatalog;
use crate::frontmatter::strip_optional_frontmatter;
use crate::{Mutation, RenderHooks, apply_frontmatter_mutations};

/// Options layered around the core render pipeline.
#[derive(Default)]
pub struct RenderOptions<'a> {
    hooks: RenderHooks<'a>,
    mutations: &'a [Mutation],
    strip_frontmatter: bool,
}

impl<'a> RenderOptions<'a> {
    /// Create default render options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Configure host hooks for rendering.
    pub fn with_hooks(mut self, hooks: RenderHooks<'a>) -> Self {
        self.hooks = hooks;
        self
    }

    /// Configure frontmatter mutations to apply after rendering.
    pub fn with_mutations(mut self, mutations: &'a [Mutation]) -> Self {
        self.mutations = mutations;
        self
    }

    /// Strip an optional leading frontmatter block after rendering.
    pub fn strip_frontmatter(mut self) -> Self {
        self.strip_frontmatter = true;
        self
    }
}

/// Render a template by replacing `{{placeholder}}` patterns and applying the
/// configured options.
pub fn render(
    template: &str,
    replacements: &[(&str, &str)],
    options: RenderOptions<'_>,
) -> Result<String> {
    let replacements: HashMap<&str, &str> = replacements.iter().copied().collect();
    let mut rendered = String::with_capacity(template.len());
    let mut cursor = 0;

    while let Some(start_rel) = template[cursor..].find("{{") {
        let start = cursor + start_rel;
        rendered.push_str(&template[cursor..start]);

        let Some(end_rel) = template[start + 2..].find("}}") else {
            rendered.push_str(&template[start..]);
            cursor = template.len();
            break;
        };
        let end = start + 2 + end_rel;
        let token = template[start + 2..end].trim();

        if token.is_empty() {
            rendered.push_str(&template[start..end + 2]);
        } else if let Some(value) = replacements.get(token) {
            rendered.push_str(value);
        } else if let Some(resolver) = options.hooks.token_resolver {
            match resolver(token)? {
                Some(value) => rendered.push_str(&value),
                None => rendered.push_str(&template[start..end + 2]),
            }
        } else {
            rendered.push_str(&template[start..end + 2]);
        }

        cursor = end + 2;
    }

    rendered.push_str(&template[cursor..]);

    if let Some(post_processor) = options.hooks.post_processor {
        rendered = post_processor(&rendered)?;
    }

    if !options.mutations.is_empty() {
        rendered = apply_frontmatter_mutations(&rendered, options.mutations);
    }

    if options.strip_frontmatter {
        rendered = strip_optional_frontmatter(&rendered);
    }

    Ok(rendered)
}

/// Load and render a template by identifier.
pub fn render_from_catalog<C: TemplateCatalog + ?Sized>(
    catalog: &C,
    template_id: &str,
    replacements: &[(&str, &str)],
    options: RenderOptions<'_>,
) -> Result<String> {
    let template = catalog.load(template_id)?;
    render(&template, replacements, options)
}

#[cfg(test)]
mod tests {
    use anyhow::{Result, anyhow};

    use crate::{
        MemoryTemplateCatalog, Mutation, RenderHooks, RenderOptions, render, render_from_catalog,
    };

    #[test]
    fn render_replaces_placeholders() -> Result<()> {
        let result = render(
            "Hello {{name}}, created at {{created_at}}",
            &[("name", "World"), ("created_at", "2026-03-02T00:00:00")],
            RenderOptions::default(),
        )?;
        assert_eq!(result, "Hello World, created at 2026-03-02T00:00:00");
        Ok(())
    }

    #[test]
    fn render_with_options_resolves_missing_tokens() -> Result<()> {
        let hooks = RenderHooks::new()
            .with_token_resolver(&|token| Ok((token == "name").then(|| "World".to_string())));

        let result = render(
            "Hello {{name}}",
            &[],
            RenderOptions::new().with_hooks(hooks),
        )?;
        assert_eq!(result, "Hello World");
        Ok(())
    }

    #[test]
    fn render_with_options_propagates_failures() {
        let hooks = RenderHooks::new().with_token_resolver(&|_| Err(anyhow!("boom")));
        let err = render(
            "Hello {{name}}",
            &[],
            RenderOptions::new().with_hooks(hooks),
        )
        .unwrap_err();
        assert!(err.to_string().contains("boom"));
    }

    #[test]
    fn render_from_catalog_loads_named_template() -> Result<()> {
        let mut catalog = MemoryTemplateCatalog::new();
        catalog.insert("hello", "Hello {{name}}");

        let result = render_from_catalog(
            &catalog,
            "hello",
            &[("name", "World")],
            RenderOptions::default(),
        )?;
        assert_eq!(result, "Hello World");
        Ok(())
    }

    #[test]
    fn render_from_catalog_applies_mutations() -> Result<()> {
        let mut catalog = MemoryTemplateCatalog::new();
        catalog.insert(
            "story",
            "\
---
id: TEST0001
title: {{title}}
status: draft
---

# {{title}}
",
        );

        let result = render_from_catalog(
            &catalog,
            "story",
            &[("title", "Example")],
            RenderOptions::new().with_mutations(&[
                Mutation::set("status", "planned"),
                Mutation::set("index", "3"),
            ]),
        )?;

        assert!(result.contains("title: Example"));
        assert!(result.contains("status: planned"));
        assert!(result.contains("index: 3"));
        assert!(!result.contains("status: draft"));
        Ok(())
    }

    #[test]
    fn render_can_strip_frontmatter() -> Result<()> {
        let result = render(
            "\
---
created_at: {{created_at}}
---

# Body
",
            &[("created_at", "2026-03-02T00:00:00")],
            RenderOptions::new().strip_frontmatter(),
        )?;
        assert_eq!(result, "# Body\n");
        Ok(())
    }

    #[test]
    fn render_from_catalog_can_strip_frontmatter() -> Result<()> {
        let mut catalog = MemoryTemplateCatalog::new();
        catalog.insert(
            "body",
            "\
---
created_at: {{created_at}}
---

# Body
",
        );

        let result = render_from_catalog(
            &catalog,
            "body",
            &[("created_at", "2026-03-02T00:00:00")],
            RenderOptions::new().strip_frontmatter(),
        )?;
        assert_eq!(result, "# Body\n");
        Ok(())
    }
}
