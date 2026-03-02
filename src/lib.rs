//! Keel library root.
//!
//! Exposes normalized architecture layer roots so the binary entry point
//! can remain a thin CLI bootstrap adapter.

pub mod application;
pub mod cli;
pub mod domain;
pub mod infrastructure;
pub mod read_model;

pub use crate::cli::build_cli;

#[cfg(test)]
mod architecture_contract_tests;
#[cfg(test)]
mod cli_tests;
#[cfg(test)]
mod command_regression_tests;
#[cfg(test)]
mod drift_tests;
#[cfg(test)]
mod test_helpers;
