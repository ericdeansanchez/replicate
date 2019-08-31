//! # Generates the top-level cli.
use crate::commands;

/// Builds an `App`. This `App` is comprised of information read from cargo
/// environment variables, a list of settings, and a list of a list of all
/// supported sub-commands.
pub fn app() -> App {
    App::new(env!(stringify!(CARGO_PKG_NAME)))
        .version(env!(stringify!(CARGO_PKG_VERSION)))
        .author(env!(stringify!(CARGO_PKG_AUTHORS)))
        .about(env!(stringify!(CARGO_PKG_DESCRIPTION)))
        .settings(&[
            AppSettings::UnifiedHelpMessage,
            AppSettings::DeriveDisplayOrder,
            AppSettings::VersionlessSubcommands,
            AppSettings::AllowExternalSubcommands,
            AppSettings::SubcommandRequiredElseHelp,
        ])
        .subcommands(commands::all_sub_commands())
}