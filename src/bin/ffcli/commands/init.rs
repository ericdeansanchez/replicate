use std::env;

use ffcli::command_prelude::{App, Arg, SubCommand};
use ffcli::Result;

pub fn cli() -> App {
    SubCommand::with_name("init")
        .about("Initialize a FFCLI app.")
        .arg(
            Arg::with_name("name")
                .help("The name of the cli application")
                .required(true),
        )
}

pub fn init() {}

fn create_bin() {}

fn create_lib() {}
