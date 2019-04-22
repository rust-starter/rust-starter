#[cfg(test)]
extern crate assert_cmd;
extern crate predicates;

use assert_cmd::prelude::*;
use predicates::prelude::*;

use std::process::Command;

#[test]
fn test_cli() {
    let mut cmd = Command::cargo_bin("rust-starter").unwrap();
    cmd.assert().failure();
}

#[test]
fn test_version() {
    let expected_version = "rust-starter 0.0.1\n";
    let mut cmd = Command::cargo_bin("rust-starter").unwrap();
    cmd.arg("--version").assert().stdout(expected_version);
}

#[test]
fn test_hazard_exit_code() {
    let mut cmd = Command::cargo_bin("rust-starter").unwrap();
    cmd.arg("hazard").assert().code(0);
}

