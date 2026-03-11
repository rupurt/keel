//! Binary entrypoint.
//!
//! Delegates all command parsing and dispatch to the CLI layer.

extern crate keel;

mod cli;

#[cfg(test)]
mod cli_tests;
#[cfg(test)]
mod command_regression_tests;
#[cfg(test)]
mod drift_tests;

use anyhow::Result;

fn main() -> Result<()> {
    cli::run()
}
