//! Binary entrypoint.
//!
//! Delegates all command parsing and dispatch to the CLI layer.

extern crate keel_core as keel;

mod cli;

#[cfg(test)]
mod cli_tests;
#[cfg(test)]
mod command_regression_tests;
#[cfg(test)]
mod drift_tests;

fn main() {
    if let Err(error) = cli::run() {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}
