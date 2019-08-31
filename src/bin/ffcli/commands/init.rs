use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::process::{Command, Output};

use ffcli::command_prelude::{App, Arg, SubCommand};
use ffcli::util::{fail_loudly_then_exit, ffcli_io};
use ffcli::{FfcliError, Result};

const FFCLI_APPNAME: &str = "FFCLI_APPNAME";

pub fn cli() -> App {
    SubCommand::with_name("init")
        .about("Initialize a FFCLI app.")
        .arg(
            Arg::with_name("name")
                .help("The name of the cli application")
                .required(true),
        )
}

pub fn exec(app: String) -> Result<()> {
    init(app)
}

fn init(app: String) -> Result<()> {
    match call_cargo_new(&app) {
        Ok(output) => {
            if output.status.success() {
                env::set_var(FFCLI_APPNAME, &app);
                restructure_app(app)
            } else {
                fail_loudly_then_exit(format!(
                    "error: call to `cargo new {}` failed, exiting...",
                    &app
                ))
            }
        }
        Err(e) => Err(FfcliError::Io(e)),
    }
}

fn call_cargo_new(app: &str) -> io::Result<Output> {
    Command::new("cargo").arg("new").arg(app).output()
}

fn restructure_app(app: String) -> Result<()> {
    let current_dir = env::current_dir()?;
    let src = current_dir.join(&app).join("src");
    env::set_current_dir(&src)?;
    ffcli_io::remove_file(src.join("main.rs"))?;
    create_bin(&app)?;
    create_lib(&app)?;
    update_cargo_toml(current_dir.join(&app).join("Cargo.toml"))?;
    env::set_current_dir(&current_dir)?;
    Ok(())
}

fn create_bin(app: &str) -> Result<()> {
    let bin = Path::new("bin").join(&app);
    fs::create_dir_all(&bin)?;
    populate_bin(&bin)?;
    Ok(())
}

fn populate_bin<P: AsRef<Path>>(path: P) -> Result<()> {
    write_main_rs(path.as_ref())?;
    write_cli_rs(path.as_ref())?;
    create_commands(path.as_ref())?;
    Ok(())
}

fn write_main_rs<P: AsRef<Path>>(path: P) -> Result<()> {
    let main = path.as_ref().join("main.rs");
    let app_name = get_app_name();
    let contents = format!(
        r#"use std::process::exit;

use {AppName}::Result;

// Module Declarations.
mod cli;
mod commands;

fn main() -> Result<()> {{
    Ok(run(cli::app())?)
}}

/// Executes a cli app. This function parses the command line arguments and
/// maps a given command to _its_ executor.
fn run(app: clap::App<'static, 'static>) -> Result<()> {{
    match app.get_matches().subcommand() {{
        _ => {{
            exit(1);
        }}
    }}
}}
"#,
        AppName = app_name
    );

    Ok(ffcli_io::write(&main, contents.as_bytes())?)
}

fn write_cli_rs<P: AsRef<Path>>(path: P) -> Result<()> {
    let cli = path.as_ref().join("cli.rs");
    let app_name = get_app_name();
    let contents = format!(
        r#"//! # Generates the top-level cli.
use crate::commands;
use {AppName}::command_prelude::*;

/// Builds an `App`. This `App` is comprised of information read from cargo
/// environment variables, a list of settings, and a list of a list of all
/// supported sub-commands.
pub fn app() -> App {{
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .settings(&[
            AppSettings::UnifiedHelpMessage,
            AppSettings::DeriveDisplayOrder,
            AppSettings::VersionlessSubcommands,
            AppSettings::AllowExternalSubcommands,
            AppSettings::SubcommandRequiredElseHelp,
        ])
        .subcommands(commands::all_sub_commands())
}}
"#,
        AppName = app_name
    );
    Ok(ffcli_io::write(&cli, contents.as_bytes())?)
}

fn create_commands<P: AsRef<Path>>(path: P) -> Result<()> {
    let commands = path.as_ref().join("commands");
    fs::create_dir_all(&commands)?;
    write_commands_mod_rs(commands.join("mod.rs"))?;
    write_command_init_rs(commands.join("init.rs"))?;
    Ok(())
}

fn write_commands_mod_rs<P: AsRef<Path>>(path: P) -> Result<()> {
    let app_name = get_app_name();
    let contents = format!(
        r#"use {AppName}::command_prelude::*;

pub fn all_sub_commands() -> Vec<App> {{
    vec![init::cli()]
}}

pub mod init;
"#,
        AppName = app_name
    );
    Ok(ffcli_io::write(path.as_ref(), contents.as_bytes())?)
}

fn write_command_init_rs<P: AsRef<Path>>(path: P) -> Result<()> {
    let app_name = get_app_name();
    let contents = format!(
        r#"use {AppName}::command_prelude::{{App, Arg, SubCommand}};

pub fn cli() -> App {{
    SubCommand::with_name("init")
        .about("Example init command.")
        .arg(
            Arg::with_name("name")
                .help("The name of argument to init.")
                .required(true),
        )
}}"#,
        AppName = app_name
    );
    Ok(ffcli_io::write(path.as_ref(), contents.as_bytes())?)
}

fn create_lib(app: &str) -> Result<()> {
    let lib = Path::new(&app);
    fs::create_dir_all(&lib)?;
    populate_lib(&lib)?;
    Ok(())
}

fn populate_lib<P: AsRef<Path>>(path: P) -> Result<()> {
    write_lib_rs(path.as_ref())?;
    write_util(path.as_ref())?;
    Ok(())
}

fn write_lib_rs<P: AsRef<Path>>(path: P) -> Result<()> {
    let lib = path.as_ref().join("lib.rs");
    let app_name = get_app_name();
    let contents = format!(
        r#"// Module declarations.
pub mod util;

/// Re-exports.
pub use util::command_prelude;
// Note: You should covert the AppName to upper camel case,
// e.g. AppNameError
pub use util::errors::{{ {AppName}Error, Result}};
"#,
        AppName = app_name
    );
    Ok(ffcli_io::write(&lib, contents.as_bytes())?)
}

fn write_util<P: AsRef<Path>>(path: P) -> Result<()> {
    let util = path.as_ref().join("util");
    fs::create_dir_all(&util)?;
    populate_util(&util)?;
    Ok(())
}

fn populate_util<P: AsRef<Path>>(path: P) -> Result<()> {
    write_util_mod_rs(&path)?;
    write_command_prelude_rs(&path)?;
    write_errors_rs(&path)?;
    Ok(())
}

fn write_util_mod_rs<P: AsRef<Path>>(path: P) -> Result<()> {
    let mod_rs = path.as_ref().join("mod.rs");
    let contents = r#"/// Utility module declarations.
pub mod command_prelude;
pub mod errors;
"#;

    Ok(ffcli_io::write(&mod_rs, contents.as_bytes())?)
}

fn write_command_prelude_rs<P: AsRef<Path>>(path: P) -> Result<()> {
    let prelude = path.as_ref().join("command_prelude.rs");
    let contents = r#"/// Re-exporting `std::process`.
pub use std::process;

/// Re-exporting commonly necessary types from [`clap`].(https://github.com/clap-rs/clap)
pub use clap::{AppSettings, Arg, ArgMatches, SubCommand};

/// Type alias for a `clap::App`.
pub type App = clap::App<'static, 'static>;
"#;

    Ok(ffcli_io::write(&prelude, contents.as_bytes())?)
}

fn write_errors_rs<P: AsRef<Path>>(path: P) -> Result<()> {
    let errors = path.as_ref().join("errors.rs");
    let app_name = get_app_name();
    let contents = format!(
        r#"//! Primary error structures for {AppName}.
use std::io;

// Note: ffcli can bring in a third-party crate to title-case
// AppName, however, if you're using RLS, then this will be
// caught by the linter.

/// Error types for {AppName}.
#[derive(Debug)]
pub enum {AppName}Error {{
    Io(io::Error),
}}

impl From<io::Error> for {AppName}Error {{
    fn from(err: io::Error) -> {AppName}Error {{
        {AppName}Error::Io(err)
    }}
}}

/// Custom result type for {AppName}.
pub type Result<T> = std::result::Result<T, {AppName}Error>;
"#,
        AppName = app_name
    );
    Ok(ffcli_io::write(&errors, contents.as_bytes())?)
}

fn update_cargo_toml<P: AsRef<Path>>(path: P) -> Result<()> {
    let app_name = get_app_name();
    let contents = format!(
        r#"# ffcli aims to be reasonably generic, if you want
# to use a specific version of clap, you should change the
# following to that version (e.g. clap = "2.33.0").
clap = "*"

[lib]
name = "{AppName}"
path = "src/{AppName}/lib.rs"
"#,
        AppName = app_name
    );

    if fs::metadata(path.as_ref()).is_ok() {
        ffcli_io::append(path.as_ref(), contents.as_bytes())?;
    } else {
        fail_loudly_then_exit(format!("error: failed to retrieve Cargo.toml"))?;
    }
    Ok(())
}

fn get_app_name() -> String {
    match env::var_os(FFCLI_APPNAME) {
        Some(os) => os
            .to_str()
            .map(String::from)
            .unwrap_or(String::from("AppName")),
        None => String::from("AppName"),
    }
}
