use assert_cmd::Command;
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn test_no_player() -> TestResult {
    let mut cmd = Command::cargo_bin("player")?;
    cmd.arg("list_player").assert().success()
    .stdout("Player 1 : N/A\nPlayer 2 : N/A");
     Ok(())
}

#[test]
fn test_onename_player() -> TestResult {
    let mut cmd = Command::cargo_bin("player")?;
    cmd.arg("list_player").arg("Mike").assert().success()
    .stdout("Player 1 : Mike\nPlayer 2 : N/A");
     Ok(())
}

#[test]
fn test_twoname_player() -> TestResult {
    let mut cmd = Command::cargo_bin("player")?;
    cmd.arg("list_player").arg("Mike").arg("Leo").assert().success()
    .stdout("Player 1 : Mike\nPlayer 2 : Leo");
     Ok(())
}

#[test]
fn test_morethantwo_name_player() -> TestResult {
    let mut cmd = Command::cargo_bin("player")?;
    cmd.arg("list_player").arg("Mike").arg("Leo").arg("Ralph")
    .assert().success()
    .stdout("Player 1 : Mike\nPlayer 2 : Leo");
     Ok(())
}