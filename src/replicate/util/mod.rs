//! Utilities for replicator.
use std::io::{self, Write};
use std::process::exit;

use errors::Result;

pub fn fail_loudly_then_exit(msg: String) -> Result<()> {
    io::stderr().write(msg.as_bytes())?;
    exit(1);
}

/// Utility module declarations.
pub mod command_prelude;
pub mod errors;
pub mod replicate_io;
