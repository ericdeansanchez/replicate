//! Aggregates and exposes all ffcli commands' cli's.
use ffcli::command_prelude::*;

pub fn all_sub_commands() -> Vec<App> {
    vec![init::cli()]
}

pub mod init;
