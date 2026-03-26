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
//!
//! Stable extension points:
//! - [`TemplateCatalog`] for host-owned template lookup.
//! - [`RenderHooks`] and [`RenderOptions`] for render-time customization.
//! - [`render`] and [`render_from_catalog`] as the core document render entrypoints.
//! - [`Mutation`] and [`apply_frontmatter_mutations`] for generic frontmatter edits.
//!
//! Host-owned responsibilities:
//! - Template inventory, filesystem layout, and cache strategy.
//! - Project-specific document wrappers or convenience helpers.
//! - Domain semantics encoded in replacements, mutations, or post-processing.

mod catalog;
mod frontmatter;
mod hooks;
mod render;

pub use catalog::{MemoryTemplateCatalog, TemplateCatalog};
pub use frontmatter::{Mutation, apply_frontmatter_mutations};
pub use hooks::RenderHooks;
pub use render::{RenderOptions, render, render_from_catalog};
