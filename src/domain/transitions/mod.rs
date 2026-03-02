//! Unified state transition engine for keel.
//!
//! This module provides a declarative approach to state transitions,
//! eliminating code duplication across command files while maintaining
//! consistent behavior aligned with the 2-queue pull system.
//!
//! ## Story Transitions
//! Use `execute()` with `TransitionSpec` for story state changes.
//!
//! ## Bearing Transitions
//! Use `bearing::execute()` with `BearingTransitionSpec` for bearing
//! state changes that may include side effects (file creation).

mod engine;
mod frontmatter;
mod spec;

// Bearing transition support
mod bearing_engine;
mod bearing_spec;

// Story transition exports
pub use engine::execute;
// pub use engine::{execute_with_body_transform, execute_with_validate};
pub use frontmatter::update_frontmatter;
pub use spec::{TimestampUpdates, TransitionSpec, transitions};

// Bearing transition exports
pub mod bearing {
    pub use super::bearing_engine::execute;
    pub use super::bearing_spec::bearing_transitions;
}
