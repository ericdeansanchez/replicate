use std::env;
use std::fs;
use std::io;
use std::path::Path;
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

pub fn exec(app: String) -> Result<()> {
    init(app)
}

fn init(app: String) -> Result<()> {
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
    let app = env::current_dir()?.join(&app);
    create_bin(&app)?;
    create_lib(&app)?;
    Ok(())
}

fn create_bin<P: AsRef<Path>>(app: P) -> Result<()> {
    fs::create_dir_all(app.as_ref().join("src").join("bin"))?;
    Ok(())
}

fn create_lib<P: AsRef<Path>>(app: P) -> Result<()> {
    Ok(())
}
