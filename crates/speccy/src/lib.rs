//! Reusable markdown template rendering helpers.
//!
//! `speccy` owns generic placeholder rendering, template catalog loading,
//! fallible host hooks, and frontmatter mutation without depending on Keel.
//!
//! Boundary:
//! - Host projects own template inventory and decide how templates are loaded.
//! - `speccy` owns generic text rendering, generic frontmatter mutation, and
//!   fallible extension hooks that operate on plain strings.
//! - Project-specific document semantics stay in the host adapter layer.

use std::collections::{BTreeMap, HashMap, HashSet};

use anyhow::{Result, anyhow};

type TokenResolver<'a> = dyn Fn(&str) -> Result<Option<String>> + 'a;
type PostProcessor<'a> = dyn Fn(&str) -> Result<String> + 'a;

/// A fallible template source owned by the host project.
pub trait TemplateCatalog {
    /// Load a template body by identifier.
    fn load(&self, template_id: &str) -> Result<String>;
}

/// In-memory template catalog for tests and simple embedders.
#[derive(Debug, Clone, Default)]
pub struct MemoryTemplateCatalog {
    templates: BTreeMap<String, String>,
}

impl MemoryTemplateCatalog {
    /// Create an empty catalog.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert or replace a template body by identifier.
    pub fn insert(
        &mut self,
        template_id: impl Into<String>,
        template: impl Into<String>,
    ) -> Option<String> {
        self.templates.insert(template_id.into(), template.into())
    }
}

impl TemplateCatalog for MemoryTemplateCatalog {
    fn load(&self, template_id: &str) -> Result<String> {
        self.templates
            .get(template_id)
            .cloned()
            .ok_or_else(|| anyhow!("template not found: {template_id}"))
    }
}

/// Fallible host hooks layered around rendering.
#[derive(Default)]
pub struct RenderHooks<'a> {
    token_resolver: Option<&'a TokenResolver<'a>>,
    post_processor: Option<&'a PostProcessor<'a>>,
}

impl<'a> RenderHooks<'a> {
    /// Create an empty hook set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Resolve tokens that are not supplied in the explicit replacements list.
    pub fn with_token_resolver(mut self, token_resolver: &'a TokenResolver<'a>) -> Self {
        self.token_resolver = Some(token_resolver);
        self
    }

    /// Apply a final fallible transform to the rendered document.
    pub fn with_post_processor(mut self, post_processor: &'a PostProcessor<'a>) -> Self {
        self.post_processor = Some(post_processor);
        self
    }
}

/// Render a template by replacing `{{placeholder}}` patterns.
pub fn render(template: &str, replacements: &[(&str, &str)]) -> String {
    render_with_hooks(template, replacements, &RenderHooks::default())
        .expect("basic rendering without hooks should be infallible")
}

/// Render a template with fallible host hooks.
pub fn render_with_hooks(
    template: &str,
    replacements: &[(&str, &str)],
    hooks: &RenderHooks<'_>,
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
        } else if let Some(resolver) = hooks.token_resolver {
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

    if let Some(post_processor) = hooks.post_processor {
        post_processor(&rendered)
    } else {
        Ok(rendered)
    }
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
        apply_frontmatter_mutations(&rendered, mutations)
    }
}

/// Render a template with hooks, then apply frontmatter mutations.
pub fn render_with_hooks_and_mutations(
    template: &str,
    replacements: &[(&str, &str)],
    mutations: &[Mutation],
    hooks: &RenderHooks<'_>,
) -> Result<String> {
    let rendered = render_with_hooks(template, replacements, hooks)?;
    Ok(if mutations.is_empty() {
        rendered
    } else {
        apply_frontmatter_mutations(&rendered, mutations)
    })
}

/// Load and render a template by identifier.
pub fn render_from_catalog<C: TemplateCatalog>(
    catalog: &C,
    template_id: &str,
    replacements: &[(&str, &str)],
) -> Result<String> {
    render_from_catalog_with_hooks(catalog, template_id, replacements, &RenderHooks::default())
}

/// Load and render a template by identifier with fallible hooks.
pub fn render_from_catalog_with_hooks<C: TemplateCatalog>(
    catalog: &C,
    template_id: &str,
    replacements: &[(&str, &str)],
    hooks: &RenderHooks<'_>,
) -> Result<String> {
    let template = catalog.load(template_id)?;
    render_with_hooks(&template, replacements, hooks)
}

/// Load, render, and mutate a template by identifier.
pub fn render_from_catalog_with_mutations<C: TemplateCatalog>(
    catalog: &C,
    template_id: &str,
    replacements: &[(&str, &str)],
    mutations: &[Mutation],
    hooks: &RenderHooks<'_>,
) -> Result<String> {
    let rendered = render_from_catalog_with_hooks(catalog, template_id, replacements, hooks)?;
    Ok(if mutations.is_empty() {
        rendered
    } else {
        apply_frontmatter_mutations(&rendered, mutations)
    })
}

/// Render a template and strip an optional leading frontmatter block.
pub fn render_body(template: &str, replacements: &[(&str, &str)]) -> String {
    strip_optional_frontmatter(&render(template, replacements))
}

/// Render a template with hooks and strip an optional leading frontmatter block.
pub fn render_body_with_hooks(
    template: &str,
    replacements: &[(&str, &str)],
    hooks: &RenderHooks<'_>,
) -> Result<String> {
    Ok(strip_optional_frontmatter(&render_with_hooks(
        template,
        replacements,
        hooks,
    )?))
}

/// Load a template by identifier, render it, and strip an optional leading
/// frontmatter block.
pub fn render_body_from_catalog<C: TemplateCatalog>(
    catalog: &C,
    template_id: &str,
    replacements: &[(&str, &str)],
    hooks: &RenderHooks<'_>,
) -> Result<String> {
    Ok(strip_optional_frontmatter(&render_from_catalog_with_hooks(
        catalog,
        template_id,
        replacements,
        hooks,
    )?))
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

/// A generic markdown frontmatter mutation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mutation {
    Set { key: String, value: String },
    Remove { key: String },
}

impl Mutation {
    pub fn set(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self::Set {
            key: key.into(),
            value: value.into(),
        }
    }

    pub fn remove(key: impl Into<String>) -> Self {
        Self::Remove { key: key.into() }
    }

    fn key(&self) -> &str {
        match self {
            Self::Set { key, .. } | Self::Remove { key } => key,
        }
    }
}

/// Apply frontmatter mutations to markdown content.
///
/// - Existing keys are replaced or removed in-place.
/// - Missing `set` keys are inserted before the closing frontmatter delimiter.
pub fn apply_frontmatter_mutations(content: &str, mutations: &[Mutation]) -> String {
    let mut result = String::new();
    let mut in_frontmatter = false;
    let mut delimiter_count = 0;
    let mut handled = HashSet::new();
    let mut handled_subs = HashSet::new();

    let mut nested_mutations: HashMap<String, Vec<(String, String)>> = HashMap::new();

    for mutation in mutations {
        if let Mutation::Set { key, value } = mutation
            && let Some((section, subkey)) = key.split_once('.')
        {
            nested_mutations
                .entry(section.to_string())
                .or_default()
                .push((subkey.to_string(), value.clone()));
        }
    }

    let mut current_section: Option<String> = None;

    for line in content.lines() {
        if line == "---" {
            delimiter_count += 1;
            in_frontmatter = delimiter_count == 1;

            if delimiter_count == 2 {
                if let Some(section) = &current_section
                    && let Some(subs) = nested_mutations.get(section)
                {
                    for (subkey, value) in subs {
                        let full_key = format!("{}.{}", section, subkey);
                        if !handled_subs.contains(&full_key) {
                            result.push_str(&format!("  {subkey}: {value}\n"));
                            handled_subs.insert(full_key);
                        }
                    }
                }

                for mutation in mutations {
                    if let Mutation::Set { key, value } = mutation
                        && !key.contains('.')
                        && !handled.contains(key)
                    {
                        result.push_str(&format!("{key}: {value}\n"));
                    }
                }

                for (section, subs) in &nested_mutations {
                    if !handled.contains(section) {
                        result.push_str(&format!("{section}:\n"));
                        for (subkey, value) in subs {
                            result.push_str(&format!("  {subkey}: {value}\n"));
                        }
                    }
                }
            }

            result.push_str(line);
            result.push('\n');
            continue;
        }

        if in_frontmatter {
            if let Some(key) = frontmatter_key(line) {
                if let Some(prev_section) = &current_section
                    && let Some(subs) = nested_mutations.get(prev_section)
                {
                    for (subkey, value) in subs {
                        let full_key = format!("{}.{}", prev_section, subkey);
                        if !handled_subs.contains(&full_key) {
                            result.push_str(&format!("  {subkey}: {value}\n"));
                            handled_subs.insert(full_key);
                        }
                    }
                }

                current_section = Some(key.to_string());

                if let Some(mutation) = lookup_mutation(mutations, key) {
                    handled.insert(key.to_string());
                    if let Mutation::Set { key, value } = mutation {
                        result.push_str(&format!("{key}: {value}\n"));
                    }
                    continue;
                }
            } else if line.starts_with("  ")
                && let Some(section) = &current_section
                && let Some(subs) = nested_mutations.get(section)
            {
                let trimmed = line.trim();
                if let Some((subkey, _)) = trimmed.split_once(':') {
                    let subkey = subkey.trim();
                    if let Some((_, value)) = subs.iter().find(|(key, _)| key == subkey) {
                        let full_key = format!("{}.{}", section, subkey);
                        result.push_str(&format!("  {subkey}: {value}\n"));
                        handled_subs.insert(full_key);
                        handled.insert(section.clone());
                        continue;
                    }
                }
            }
        }

        result.push_str(line);
        result.push('\n');
    }

    result
}

fn lookup_mutation<'a>(mutations: &'a [Mutation], key: &str) -> Option<&'a Mutation> {
    mutations
        .iter()
        .rev()
        .find(|mutation| mutation.key() == key)
}

fn frontmatter_key(line: &str) -> Option<&str> {
    let (key, _) = line.split_once(':')?;
    let key = key.trim();
    if key.is_empty() { None } else { Some(key) }
}

#[cfg(test)]
mod tests {
    use anyhow::{Result, anyhow};

    use super::{
        MemoryTemplateCatalog, Mutation, RenderHooks, apply_frontmatter_mutations, render,
        render_body, render_body_from_catalog, render_from_catalog,
        render_from_catalog_with_mutations, render_with_hooks,
    };

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
    fn render_with_hooks_resolves_missing_tokens() -> Result<()> {
        let hooks = RenderHooks::new()
            .with_token_resolver(&|token| Ok((token == "name").then(|| "World".to_string())));

        let result = render_with_hooks("Hello {{name}}", &[], &hooks)?;
        assert_eq!(result, "Hello World");
        Ok(())
    }

    #[test]
    fn render_with_hooks_propagates_failures() {
        let hooks = RenderHooks::new().with_token_resolver(&|_| Err(anyhow!("boom")));
        let err = render_with_hooks("Hello {{name}}", &[], &hooks).unwrap_err();
        assert!(err.to_string().contains("boom"));
    }

    #[test]
    fn render_from_catalog_loads_named_template() -> Result<()> {
        let mut catalog = MemoryTemplateCatalog::new();
        catalog.insert("hello", "Hello {{name}}");

        let result = render_from_catalog(&catalog, "hello", &[("name", "World")])?;
        assert_eq!(result, "Hello World");
        Ok(())
    }

    #[test]
    fn render_from_catalog_with_mutations_updates_frontmatter_once() -> Result<()> {
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

        let result = render_from_catalog_with_mutations(
            &catalog,
            "story",
            &[("title", "Example")],
            &[
                Mutation::set("status", "planned"),
                Mutation::set("index", "3"),
            ],
            &RenderHooks::default(),
        )?;

        assert!(result.contains("title: Example"));
        assert!(result.contains("status: planned"));
        assert!(result.contains("index: 3"));
        assert!(!result.contains("status: draft"));
        Ok(())
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

    #[test]
    fn render_body_from_catalog_supports_loaded_templates() -> Result<()> {
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

        let result = render_body_from_catalog(
            &catalog,
            "body",
            &[("created_at", "2026-03-02T00:00:00")],
            &RenderHooks::default(),
        )?;
        assert_eq!(result, "# Body\n");
        Ok(())
    }

    const SAMPLE: &str = r#"---
id: FEAT0001
title: Test Story
status: backlog
---

# Story
"#;

    #[test]
    fn set_replaces_existing_key() {
        let updated =
            apply_frontmatter_mutations(SAMPLE, &[Mutation::set("status", "in-progress")]);
        assert!(updated.contains("status: in-progress"));
        assert!(!updated.contains("status: backlog"));
    }

    #[test]
    fn set_inserts_missing_key_before_closing_delimiter() {
        let updated = apply_frontmatter_mutations(
            SAMPLE,
            &[
                Mutation::set("updated_at", "2026-03-02T00:00:00"),
                Mutation::set("scope", "epic/voyage"),
            ],
        );
        assert!(updated.contains("updated_at: 2026-03-02T00:00:00"));
        assert!(updated.contains("scope: epic/voyage"));
    }

    #[test]
    fn remove_deletes_existing_key() {
        let with_scope = r#"---
id: FEAT0001
status: backlog
scope: epic/voyage
---
Body
"#;
        let updated = apply_frontmatter_mutations(with_scope, &[Mutation::remove("scope")]);
        assert!(!updated.contains("scope: epic/voyage"));
    }

    #[test]
    fn last_duplicate_mutation_wins() {
        let updated = apply_frontmatter_mutations(
            SAMPLE,
            &[
                Mutation::set("status", "in-progress"),
                Mutation::set("status", "done"),
            ],
        );
        assert!(updated.contains("status: done"));
        assert!(!updated.contains("status: in-progress"));
    }
}
