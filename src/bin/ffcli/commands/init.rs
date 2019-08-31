use std::env;
use std::io;
use std::process::{Command, Output};

use ffcli::command_prelude::{App, Arg, SubCommand};
use ffcli::util::fail_loudly_then_exit;
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
