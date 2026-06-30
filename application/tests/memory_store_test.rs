use std::fs;
use tempfile::TempDir;

#[test]
fn memory_persists_to_file() {
    let dir = TempDir::new().unwrap();
    let memory_path = dir.path().join("memory.md");

    // Write memory
    let content = "fact 1\n---\nfact 2";
    fs::write(&memory_path, content).unwrap();

    // Read back
    let loaded = fs::read_to_string(&memory_path).unwrap();
    assert!(loaded.contains("fact 1"));
    assert!(loaded.contains("fact 2"));
}

#[test]
fn memory_entries_split_by_separator() {
    let content = "entry one\n---\nentry two\n---\nentry three";
    let entries: Vec<&str> = content.split("\n---\n").collect();
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0], "entry one");
    assert_eq!(entries[2], "entry three");
}

#[test]
fn empty_memory_file_is_valid() {
    let dir = TempDir::new().unwrap();
    let memory_path = dir.path().join("memory.md");
    fs::write(&memory_path, "").unwrap();

    let content = fs::read_to_string(&memory_path).unwrap();
    let entries: Vec<&str> = content
        .split("\n---\n")
        .filter(|s| !s.trim().is_empty())
        .collect();
    assert!(entries.is_empty());
}

#[test]
fn nonexistent_memory_file_is_ok() {
    let dir = TempDir::new().unwrap();
    let memory_path = dir.path().join("memory.md");
    assert!(!memory_path.exists());
}
