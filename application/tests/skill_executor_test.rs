use std::process::Command;

#[test]
fn executor_runs_echo_command() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("echo hello world")
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "hello world");
    assert!(output.status.success());
}

#[test]
fn executor_substitutes_query_placeholder() {
    let template = "echo searching for: {query}";
    let query = "rust programming";
    let cmd = template.replace("{query}", query);

    let output = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("rust programming"));
}

#[test]
fn executor_captures_stderr_on_failure() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("echo 'error msg' >&2 && exit 1")
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("error msg"));
}
