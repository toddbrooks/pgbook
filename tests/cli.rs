use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

// Current tests fail...WIP

#[test]
#[ignore]
fn fails_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("pgbook")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
#[ignore]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("pgbook")?;
    cmd.arg("run").assert().success();
    Ok(())
}
