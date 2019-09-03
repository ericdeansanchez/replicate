//! Aggregates and exposes all replicator commands' cli's.
use replicate::command_prelude::*;

pub fn all_sub_commands() -> Vec<App> {
    vec![init::cli()]
}

pub mod init;
