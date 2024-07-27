use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn run() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello").assert().success();
}

#[test]
fn hello1() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    let expected = fs::read_to_string("tests/expected/hello1.txt").unwrap();
    cmd.arg("Hello there").assert().success().stdout(expected);
}
