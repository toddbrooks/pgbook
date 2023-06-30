use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn fails_no_args() {
    let mut cmd = Command::cargo_bin("pgbook").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
#[ignore]
fn runs() {
    let mut cmd = Command::cargo_bin("pgbook").unwrap();
    cmd.arg("run").assert().success();
}
