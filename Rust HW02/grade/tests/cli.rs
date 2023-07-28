use assert_cmd::Command;
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn grade_Excellent() {
    let mut cmd = Command::cargo_bin("grade").unwrap();
    cmd.arg("98");
    cmd.assert().success().stdout("Excellent with A+");
}

#[test]
fn grade_A() {
    let mut cmd = Command::cargo_bin("grade").unwrap();
    cmd.arg("85");
    cmd.assert().success().stdout("A");
}

#[test]
fn grade_B() {
    let mut cmd = Command::cargo_bin("grade").unwrap();
    cmd.arg("78");
    cmd.assert().success().stdout("B");
}

#[test]
fn grade_C() {
    let mut cmd = Command::cargo_bin("grade").unwrap();
    cmd.arg("66");
    cmd.assert().success().stdout("C");
}

#[test]
fn grade_D() {
    let mut cmd = Command::cargo_bin("grade").unwrap();
    cmd.arg("55");
    cmd.assert().success().stdout("D");
}

#[test]
fn grade_F() {
    let mut cmd = Command::cargo_bin("grade").unwrap();
    cmd.arg("6");
    cmd.assert().success().stdout("Failed with F");
}

#[test]
fn grade_err() {
    let mut cmd = Command::cargo_bin("grade").unwrap();
    cmd.arg("420");
    cmd.assert().success().stdout("Invalid score");
}