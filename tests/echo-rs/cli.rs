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
fn runs() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("Hi there").assert().success().stdout("Hi there\n");
}

#[test]
fn one_word_no_flag() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("Hello").assert().success().stdout("Hello\n");
}

#[test]
fn one_word_with_n_flag() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("-n")
        .arg("Hello")
        .assert()
        .success()
        .stdout("Hello");
}

#[test]
fn two_words_no_flag() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("Hello there")
        .assert()
        .success()
        .stdout("Hello there\n");
}

#[test]
fn two_words_with_n_flag() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("-n")
        .arg("Hello there")
        .assert()
        .success()
        .stdout("Hello there");
}

#[test]
fn word_flag_no_space() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("-nHello")
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}

fn compare_output_arr(outfile: &str, args_passed: &[&str]) {
    let expected = fs::read_to_string(outfile).expect("can't read the dang file");
    let mut cmd = Command::cargo_bin("echor").unwrap();

    for a in args_passed {
        cmd.arg(a);
    }

    cmd.assert().success().stdout(expected);
}

#[test]
fn compare_output_hello() {
    compare_output_arr("tests/expected/hello.txt", ["Hello"].as_ref());
}

#[test]
fn compare_output_hello_n() {
    compare_output_arr("tests/expected/hello-n.txt", ["-n", "Hello"].as_ref());
}

#[test]
fn compare_output_hello_there() {
    compare_output_arr(
        "tests/expected/hello-there.txt",
        ["Hello", "there"].as_ref(),
    );
}

#[test]
fn compare_output_hello_there_n() {
    compare_output_arr(
        "tests/expected/hello-there-n.txt",
        ["-n", "Hello", "there"].as_ref(),
    );
}
