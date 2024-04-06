use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn dies_no_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("echo-rs")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn runs() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("echo-rs")?;
    cmd.arg("Hi there").assert().success().stdout("Hi there\n");
    Ok(())
}

#[test]
fn one_word_no_flag() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("echo-rs")?;
    cmd.arg("Hello").assert().success().stdout("Hello\n");
    Ok(())
}

#[test]
fn one_word_with_n_flag() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("echo-rs")?;
    cmd.arg("-n")
        .arg("Hello")
        .assert()
        .success()
        .stdout("Hello");
    Ok(())
}

#[test]
fn two_words_no_flag() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("echo-rs")?;
    cmd.arg("Hello there")
        .assert()
        .success()
        .stdout("Hello there\n");
    Ok(())
}

#[test]
fn two_words_with_n_flag() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("echo-rs")?;
    cmd.arg("-n")
        .arg("Hello there")
        .assert()
        .success()
        .stdout("Hello there");
    Ok(())
}

#[test]
fn word_flag_no_space() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("echo-rs")?;
    cmd.arg("-nHello")
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

fn compare_output_arr(
    outfile: &str,
    args_passed: &[&str],
) -> Result<(), Box<dyn std::error::Error>> {
    let expected = fs::read_to_string(outfile).expect("can't read the dang file");
    let mut cmd = Command::cargo_bin("echo-rs")?;

    for a in args_passed {
        cmd.arg(a);
    }

    cmd.assert().success().stdout(expected);
    Ok(())
}

#[test]
fn compare_output_hello() -> Result<(), Box<dyn std::error::Error>> {
    compare_output_arr("tests/expected/hello.txt", ["Hello"].as_ref())
}

#[test]
fn compare_output_hello_n() -> Result<(), Box<dyn std::error::Error>> {
    compare_output_arr("tests/expected/hello-n.txt", ["-n", "Hello"].as_ref())
}

#[test]
fn compare_output_hello_there() -> Result<(), Box<dyn std::error::Error>> {
    compare_output_arr(
        "tests/expected/hello-there.txt",
        ["Hello", "there"].as_ref(),
    )
}

#[test]
fn compare_output_hello_there_n() -> Result<(), Box<dyn std::error::Error>> {
    compare_output_arr(
        "tests/expected/hello-there-n.txt",
        ["-n", "Hello", "there"].as_ref(),
    )
}

#[test]
fn test_h_flag() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("echo-rs")?;
    cmd.arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains("USAGE"));
    Ok(())
}
