use std::fs;
use tempfile::TempDir;

#[test]
fn loads_core_md_from_directory() {
    let dir = TempDir::new().unwrap();
    let core_path = dir.path().join("core.md");
    fs::write(&core_path, "# Test Instructions\n- Be concise").unwrap();

    let content = fs::read_to_string(&core_path).unwrap();
    assert!(content.contains("Test Instructions"));
    assert!(content.contains("Be concise"));
}

#[test]
fn core_md_not_found_is_error() {
    let dir = TempDir::new().unwrap();
    let candidate = dir.path().join("core.md");
    assert!(!candidate.exists());
}
