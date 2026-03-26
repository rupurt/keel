use anyhow::Result;

type TokenResolver<'a> = dyn Fn(&str) -> Result<Option<String>> + 'a;
type PostProcessor<'a> = dyn Fn(&str) -> Result<String> + 'a;

/// Fallible host hooks layered around rendering.
#[derive(Default)]
pub struct RenderHooks<'a> {
    pub(crate) token_resolver: Option<&'a TokenResolver<'a>>,
    pub(crate) post_processor: Option<&'a PostProcessor<'a>>,
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
