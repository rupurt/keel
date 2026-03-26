use std::collections::BTreeMap;

use anyhow::{Result, anyhow};

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

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::{MemoryTemplateCatalog, TemplateCatalog};

    #[test]
    fn memory_catalog_loads_named_template() -> Result<()> {
        let mut catalog = MemoryTemplateCatalog::new();
        catalog.insert("hello", "Hello {{name}}");

        let result = catalog.load("hello")?;
        assert_eq!(result, "Hello {{name}}");
        Ok(())
    }
}
