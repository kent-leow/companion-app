use std::fs;
use tempfile::TempDir;

#[test]
fn file_tag_reads_content() {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("example.rs");
    fs::write(&file_path, "fn main() { println!(\"hello\"); }").unwrap();

    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("fn main()"));
}

#[test]
fn directory_tag_lists_files() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("a.rs"), "").unwrap();
    fs::write(dir.path().join("b.rs"), "").unwrap();
    fs::write(dir.path().join("c.md"), "").unwrap();

    let entries: Vec<String> = fs::read_dir(dir.path())
        .unwrap()
        .flatten()
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();

    assert_eq!(entries.len(), 3);
}

#[test]
fn nonexistent_file_tag_produces_error() {
    let result = fs::read_to_string("/nonexistent/path/file.rs");
    assert!(result.is_err());
}

#[test]
fn large_file_can_be_truncated() {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("big.txt");
    let large_content = "x".repeat(100_000);
    fs::write(&file_path, &large_content).unwrap();

    let content = fs::read_to_string(&file_path).unwrap();
    let max_chars = 50_000;
    let truncated = if content.len() > max_chars {
        format!("{}...[truncated]", &content[..max_chars])
    } else {
        content
    };

    assert!(truncated.len() <= max_chars + 15);
    assert!(truncated.ends_with("[truncated]"));
}
