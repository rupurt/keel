//! Shared frontmatter mutation service.
//!
//! The canonical implementation lives in `speccy`; Keel re-exports it here so
//! existing adapters can stay thin while the reusable API becomes the single
//! implementation path.

pub use speccy::{Mutation, apply_frontmatter_mutations as apply};
