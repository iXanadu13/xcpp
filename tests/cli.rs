use assert_cmd::prelude::*;
use log::info;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn invalid_std_argument() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("xcpp")?;
    cmd.current_dir("tmp");
    info!("{}", cmd.get_current_dir().unwrap().display());
    cmd.arg("new").arg("hello_cpp")
        .arg("--std=c++18"); // wrong argument
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("'c++18' isn't a valid value for '--std <std>'"));

    Ok(())
}