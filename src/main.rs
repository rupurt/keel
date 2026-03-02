//! Binary entrypoint.
//!
//! Delegates all command parsing and dispatch to the CLI layer.

use anyhow::Result;

fn main() -> Result<()> {
    keel::cli::run()
}
