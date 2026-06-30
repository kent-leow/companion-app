use std::fs;
use std::path::Path;

#[test]
fn web_search_skill_md_exists() {
    let skill_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("skills/web-search/skill.md");
    assert!(skill_path.exists());
}

#[test]
fn web_search_skill_md_has_required_sections() {
    let skill_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("skills/web-search/skill.md");
    let content = fs::read_to_string(skill_path).unwrap();

    assert!(content.contains("# web-search"));
    assert!(content.contains("## Commands"));
    assert!(content.contains("## Prompt"));
    assert!(content.contains("duckduckgo"));
    assert!(content.contains("{query}"));
}

#[test]
fn web_search_command_has_query_placeholder() {
    let skill_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("skills/web-search/skill.md");
    let content = fs::read_to_string(skill_path).unwrap();

    let in_commands = content
        .lines()
        .skip_while(|l| !l.starts_with("## Commands"))
        .skip(1)
        .take_while(|l| !l.starts_with("## "))
        .any(|l| l.contains("{query}"));

    assert!(in_commands);
}
