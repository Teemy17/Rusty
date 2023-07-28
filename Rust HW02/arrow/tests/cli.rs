use assert_cmd::Command;
type TestResult = Result<(), Box<dyn std::error::Error>>;
use predicates::str::contains;

#[test]
fn arrow_star_0() -> TestResult {
        let mut cmd = Command::cargo_bin("arrow")?;
        cmd.assert().success().stdout("");
         Ok(())
    }

#[test]
    fn test_arrow() {
        let mut cmd = Command::cargo_bin("arrow").unwrap();
        let expected_output = "*\n**\n*\n";
        cmd.arg("2").assert().success().stdout(contains(expected_output));
    }

#[test]
fn arrow_reverse_star_0() -> TestResult {
    let mut cmd = Command::cargo_bin("arrow_reverse")?;
    cmd.assert().success().stdout("");
        Ok(())
    }

#[test]
fn arrow_reverse_test() -> TestResult {
        let mut cmd = Command::cargo_bin("arrow_reverse")?;
        cmd.arg("2").assert().success().stdout(" *\n**\n *\n");
         Ok(())
    }

