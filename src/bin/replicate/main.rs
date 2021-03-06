//! Main entry point for replicate.
use std::process::exit;

use replicate::Result;

mod cli;
mod commands;

fn main() -> Result<()> {
    // run the cli app
    Ok(run(cli::app())?)
}

/// Executes a cli app. This function parses the command line arguments and
/// maps a given command to _its_ executor.
fn run(app: clap::App<'static, 'static>) -> Result<()> {
    match app.get_matches().subcommand() {
        ("cli", Some(args)) => init(args),
        _ => {
            exit(1);
        }
    }
}

/// Executes the replicate `init` command.
fn init(args: &clap::ArgMatches) -> Result<()> {
    let app_name = args
        .value_of("name")
        .map(String::from)
        .expect("bug: an app name is required");
    commands::init::exec(app_name)
}
