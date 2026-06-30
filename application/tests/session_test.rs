use tempfile::TempDir;

#[test]
fn create_session_with_id() {
    let dir = TempDir::new().unwrap();
    let session_id = "test-session-1";
    let session_dir = dir.path().join(session_id);

    assert!(!session_dir.exists());
    std::fs::create_dir_all(&session_dir).unwrap();
    assert!(session_dir.exists());
}

#[test]
fn sessions_are_isolated() {
    let dir = TempDir::new().unwrap();

    let s1_dir = dir.path().join("session-1");
    let s2_dir = dir.path().join("session-2");
    std::fs::create_dir_all(&s1_dir).unwrap();
    std::fs::create_dir_all(&s2_dir).unwrap();

    std::fs::write(s1_dir.join("memory.md"), "session 1 data").unwrap();
    std::fs::write(s2_dir.join("memory.md"), "session 2 data").unwrap();

    let m1 = std::fs::read_to_string(s1_dir.join("memory.md")).unwrap();
    let m2 = std::fs::read_to_string(s2_dir.join("memory.md")).unwrap();

    assert!(m1.contains("session 1"));
    assert!(m2.contains("session 2"));
    assert_ne!(m1, m2);
}

#[test]
fn uuid_session_ids_are_unique() {
    let id1 = uuid::Uuid::new_v4().to_string();
    let id2 = uuid::Uuid::new_v4().to_string();
    assert_ne!(id1, id2);
}
