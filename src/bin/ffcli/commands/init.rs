use std::env;
use std::io::{self, Write};
use std::process::{exit, Command, Output};

use ffcli::command_prelude::{App, Arg, SubCommand};
use ffcli::{FfcliError, Result};

pub fn cli() -> App {
    SubCommand::with_name("init")
        .about("Initialize a FFCLI app.")
        .arg(
            Arg::with_name("name")
                .help("The name of the cli application")
                .required(true),
        )
}

pub fn init(app: String) -> Result<()> {
    match call_cargo_new(&app) {
        Ok(output) => {
            if output.status.success() {
                restructure_app(app)
            } else {
                io::stderr().write_fmt(format_args!(
                    "error: call to `cargo new {}` failed, exiting...",
                    &app
                ))?;
                exit(1);
            }
        }
        Err(e) => Err(FfcliError::Io(e)),
    }
}

fn call_cargo_new(app: &str) -> io::Result<Output> {
    Command::new("cargo").arg("new").arg(app).output()
}

fn restructure_app(app: String) -> Result<()> {
    create_bin()?;
    create_lib()?;
    Ok(())
}

fn create_bin() -> Result<()> {
    Ok(())
}

fn create_lib() -> Result<()> {
    Ok(())
}
