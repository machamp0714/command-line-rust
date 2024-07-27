use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    Command::cargo_bin("echor")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    let expected = fs::read_to_string("tests/expected/hello1.txt")?; // ?演算子はResult型またはOption型を返す関数でのみ使用できる。
    Command::cargo_bin("echor")?
        .arg("Hello there")
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}
