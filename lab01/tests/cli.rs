use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("lab01").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}

#[test]
fn calc() {
    let mut cmd = Command::cargo_bin("calc").unwrap();
    cmd.assert().success().stdout("The sum is 114");
}


#[test]
fn calc_4_2() {
    let mut cmd = Command::cargo_bin("calc_4_2").unwrap();
    cmd.assert().success().stdout("34 + 80 = 114");
}


#[test]
fn shout() {
    let mut cmd = Command::cargo_bin("shout").unwrap();
    cmd.assert().success().stdout("{abcde}");
}
