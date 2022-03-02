#[cfg(test)]
extern crate assert_cmd;
extern crate predicates;

use assert_cmd::prelude::*;
use predicates::prelude::*;

use std::process::Command;

#[test]
fn test_cli() {
    let mut cmd = Command::cargo_bin("rust-starter").expect("Calling binary failed");
    cmd.assert().failure();
}

#[test]
fn test_version() {
    let expected_version = "rust-starter 2.0.0-beta\n";
    let mut cmd = Command::cargo_bin("rust-starter").expect("Calling binary failed");
    cmd.arg("--version").assert().stdout(expected_version);
}

#[test]
fn test_hazard_exit_code() {
    let mut cmd = Command::cargo_bin("rust-starter").expect("Calling binary failed");
    cmd.arg("hazard").assert().code(0);
}

#[test]
fn test_hazard_stdout() {
    let hazard_predicate =
        predicate::function(|x: &str| x == "You got it right!\n" || x == "You got it wrong!\n");
    let mut cmd = Command::cargo_bin("rust-starter").expect("Calling binary failed");
    cmd.arg("hazard").assert().stdout(hazard_predicate);
}
