use assert_cmd::Command;

#[test]
fn test_0_invaid() {
    let mut cmd = Command::

    cargo_bin("grade")
    .unwrap();

    cmd.arg("-1").assert().success()
    .stdout (

        "Invalid score"

            );
}
#[test]
fn test_49_f() {
    let mut cmd = Command::

    cargo_bin("grade")
    .unwrap();

    cmd.arg("49").assert().success()
    .stdout (

        "Failed with F"

            );
}
#[test]
fn test_60_d() {
    let mut cmd = Command::

    cargo_bin("grade")
    .unwrap();

    cmd.arg("60").assert().success()
    .stdout (

        "D"

            );
}
#[test]
fn test_70_c() {
    let mut cmd = Command::

    cargo_bin("grade")
    .unwrap();

    cmd.arg("70").assert().success()
    .stdout (

        "C"

            );
}
#[test]
fn test_80_b() {
    let mut cmd = Command::

    cargo_bin("grade")
    .unwrap();

    cmd.arg("80").assert().success()
    .stdout (

        "B"

            );
}
#[test]
fn test_94_a() {
    let mut cmd = Command::

    cargo_bin("grade")
    .unwrap();

    cmd.arg("94").assert().success()
    .stdout (

        "A"

            );
}
#[test]
fn test_100_excelent() {
    let mut cmd = Command::

    cargo_bin("grade")
    .unwrap();

    cmd.arg("100").assert().success()
    .stdout (

        "Excellent with A+"

            );
}
#[test]
fn test_101_invaid() {
    let mut cmd = Command::

    cargo_bin("grade")
    .unwrap();

    cmd.arg("101").assert().success()
    .stdout (

        "Invalid score"

            );
}