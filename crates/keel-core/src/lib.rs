//! Keel library root.
//!
//! Exposes normalized architecture layer roots so the binary entry point
//! can remain a thin CLI bootstrap adapter.

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod read_model;

pub use domain::port::{BoardStore, EntityStore};

#[cfg(test)]
mod architecture_contract_tests;
#[cfg(test)]
mod drift_tests;
#[doc(hidden)]
pub mod test_helpers;
