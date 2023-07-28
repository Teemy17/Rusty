use assert_cmd::Command;
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
    fn no_args() -> TestResult {
        let mut cmd = Command::cargo_bin("temptable")?;
        cmd.assert().failure();
         Ok(())
    }


#[test]
fn test_temptable_ascend() -> TestResult {
        let mut cmd = Command::cargo_bin("temptable")?;
        cmd.arg("0").arg("40").arg("20").assert().success().stdout("Fahr\tCelcius\n   0\t-17.8\n  20\t-6.7\n  40\t 4.4\n");
        Ok(())
    }

    #[test]
fn test_temptable_decend() -> TestResult {
        let mut cmd = Command::cargo_bin("temptable")?;
        cmd.arg("40").arg("0").arg("20").assert().success().stdout("Fahr\tCelcius\n  40\t 4.4\n  20\t-6.7\n   0\t-17.8\n");
        Ok(())
    }
