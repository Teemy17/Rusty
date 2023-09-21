use assert_cmd::Command;
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn test_sort() -> TestResult {
    let mut cmd = Command::cargo_bin("sort")?;
    cmd.arg("0").arg("2").arg("1").arg("20").arg("10").assert().success()
    .stdout("Original numbers: [0, 2, 1, 20, 10]\nAscending order: [0, 1, 2, 10, 20]\nDescending order: [20, 10, 2, 1, 0]\n");
    Ok(())
}

#[test]
fn test_sort_loop() -> TestResult {
    let mut cmd = Command::cargo_bin("sort_loop")?;
    cmd.arg("0").arg("2").arg("1").arg("20").arg("10").assert().success()
    .stdout("Ascending order: [0, 1, 2, 10, 20]\nDescending order: [20, 10, 2, 1, 0]\n");
    Ok(())
}

#[test]
fn test_sort_point() -> TestResult {
    let mut cmd = Command::cargo_bin("sort_points")?;
    cmd.arg("1").arg("5").arg("1").arg("2").arg("7").arg("3").arg("999").assert().success()
    .stdout("Points sorted in ascending order: [(1.0, 2.0), (1.0, 5.0), (7.0, 3.0)]\nPoints sorted in descending order: [(7.0, 3.0), (1.0, 5.0), (1.0, 2.0)]\n");
    Ok(())
}

#[test]
fn test_sort_point_loop() -> TestResult {
    let mut cmd = Command::cargo_bin("sort_points_loop")?;
    cmd.arg("1").arg("5").arg("1").arg("2").arg("7").arg("3").arg("999").assert().success()
    .stdout("Points sorted in ascending order: [(1.0, 2.0), (1.0, 5.0), (7.0, 3.0)]\nPoints sorted in descending order: [(7.0, 3.0), (1.0, 5.0), (1.0, 2.0)]\n");
    Ok(())
}