use std::env;
use std::io;
use std::str;

use assert_cmd::prelude::*;
use predicates::str::is_empty;
use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_resulting_cli() {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    Command::cargo_bin("replicate")
        .unwrap()
        .args(&["cli", "test_app"])
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(is_empty());
}

#[test]
fn test_base_cli_output() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    let app_name = "test_app";
    let left = format!(
        r#"{AppName} 0.1.0
{Authors}


USAGE:
    test_app <SUBCOMMAND>

OPTIONS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    init    Example init command.
    help    Prints this message or the help of the given subcommand(s)
"#,
        AppName = app_name,
        Authors = env!("CARGO_PKG_AUTHORS")
    );

    Command::cargo_bin("replicate")
        .expect("failed to unwrap in test_base_cli")
        .args(&["cli", "test_app"])
        .current_dir(&temp_dir)
        .assert()
        .success();

    env::set_current_dir(temp_dir.into_path().join("test_app"))?;

    let right = Command::new("cargo")
        .args(&["run"])
        .output()
        .and_then(|output| Ok(output.stderr))
        .map(|v| String::from(str::from_utf8(&v[..]).unwrap()))
        .unwrap();

    assert!(right.contains(&left[..]));
    env::set_current_dir(&current_dir)?;
    Ok(())
}
