use std::process::Command;
use std::time::Instant;

#[test]
fn startup_under_500ms() {
    // Note: 200ms target is for --release builds.
    // Debug builds are slower; 500ms is the relaxed threshold for CI.
    let start = Instant::now();
    let output = Command::new(env!("CARGO_BIN_EXE_companion"))
        .arg("--help")
        .output()
        .expect("failed to run companion");
    let elapsed = start.elapsed();

    assert!(output.status.success());
    assert!(
        elapsed.as_millis() < 500,
        "startup took {}ms, expected < 500ms (debug) / < 200ms (release)",
        elapsed.as_millis()
    );
}
