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
    Ok(())
}

fn write_main_rs<P: AsRef<Path>>(path: P) -> Result<()> {
    let main = path.as_ref().join("main.rs");
    let contents = r#"fn main() -> Result<()> {
    Ok(())
}    
"#;
    Ok(ffcli_io::write(&main, contents.as_bytes())?)
}

fn write_cli_rs<P: AsRef<Path>>(path: P) -> Result<()> {
    let cli = path.as_ref().join("cli.rs");
    let contents = r#"//! # Generates the top-level cli.
use crate::commands;
use ffcli::command_prelude::*;

/// Builds an `App`. This `App` is comprised of information read from cargo
/// environment variables, a list of settings, and a list of a list of all
/// supported sub-commands.
pub fn app() -> App {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("CARGO_PKG_DESCRIPTION")
        .settings(&[
            AppSettings::UnifiedHelpMessage,
            AppSettings::DeriveDisplayOrder,
            AppSettings::VersionlessSubcommands,
            AppSettings::AllowExternalSubcommands,
            AppSettings::SubcommandRequiredElseHelp,
        ])
        .subcommands(commands::all_sub_commands())
}
"#;
    Ok(ffcli_io::write(&cli, contents.as_bytes())?)
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
    let contents = r#"// Module declarations.
pub mod util;

/// Re-exports.
pub use util::command_prelude;
"#;
    Ok(ffcli_io::write(&lib, contents.as_bytes())?)
}

fn write_util<P: AsRef<Path>>(path: P) -> Result<()> {
    let util = path.as_ref().join("util");
    fs::create_dir_all(&util)?;
    populate_util(&util)?;
    Ok(())
}

fn populate_util<P: AsRef<Path>>(path: P) -> Result<()> {
    write_command_prelude_rs(&path)?;
    write_errors_rs(&path)?;
    Ok(())
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
        r#"# Be sure to add clap as a dependency.
# clap = "*"

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
