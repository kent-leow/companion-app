use std::process::Command;

#[test]
fn binary_runs_with_help() {
    let output = Command::new(env!("CARGO_BIN_EXE_companion"))
        .arg("--help")
        .output()
        .expect("failed to run companion");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("AI companion agent CLI"));
}

#[test]
fn binary_shows_version() {
    let output = Command::new(env!("CARGO_BIN_EXE_companion"))
        .arg("--version")
        .output()
        .expect("failed to run companion");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("companion"));
}
