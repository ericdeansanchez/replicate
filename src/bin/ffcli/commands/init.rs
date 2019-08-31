use std::env;
use std::io;
use std::process::{Command, ExitStatus, Output};

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
        Err(e) => Err(FfcliError::Io(e)),
        Ok(_) => restructure_app(&app),
    }
}

fn call_cargo_new(app: &str) -> io::Result<Output> {
    Command::new("cargo").arg("new").arg(app).output()
}

fn restructure_app(app: &str) -> Result<()> {
    Ok(())
}

fn create_bin() {}

fn create_lib() {}
