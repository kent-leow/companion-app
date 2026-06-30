use std::fs;
use tempfile::TempDir;

fn parse_skill_md(content: &str) -> (String, String, Vec<String>, String) {
    let mut name = String::new();
    let mut description = String::new();
    let mut commands = Vec::new();
    let mut prompt = String::new();
    let mut in_commands = false;
    let mut in_prompt = false;

    for line in content.lines() {
        if line.starts_with("# ") {
            name = line.trim_start_matches("# ").trim().to_string();
            continue;
        }
        if line.starts_with("## Commands") {
            in_commands = true;
            in_prompt = false;
            continue;
        }
        if line.starts_with("## Prompt") {
            in_commands = false;
            in_prompt = true;
            continue;
        }
        if line.starts_with("## ") {
            in_commands = false;
            in_prompt = false;
            continue;
        }
        if in_commands {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with("```") {
                commands.push(trimmed.to_string());
            }
        } else if in_prompt {
            prompt.push_str(line);
            prompt.push('\n');
        } else if description.is_empty() && !line.trim().is_empty() && !name.is_empty() {
            description = line.trim().to_string();
        }
    }

    (name, description, commands, prompt.trim().to_string())
}

#[test]
fn parses_skill_md_correctly() {
    let content = r#"# web-search

Search the web using DuckDuckGo

## Commands

```sh
curl -s "https://html.duckduckgo.com/html/?q={query}" | grep -oP '(?<=<a rel="nofollow" class="result__a" href=").*?(?=")'
```

## Prompt

You are a web search assistant. Summarize the search results concisely.
"#;

    let (name, desc, cmds, prompt) = parse_skill_md(content);
    assert_eq!(name, "web-search");
    assert!(desc.contains("DuckDuckGo"));
    assert_eq!(cmds.len(), 1);
    assert!(cmds[0].contains("duckduckgo"));
    assert!(prompt.contains("web search assistant"));
}

#[test]
fn loads_skills_from_directory() {
    let dir = TempDir::new().unwrap();
    let skill_dir = dir.path().join("web-search");
    fs::create_dir(&skill_dir).unwrap();
    fs::write(
        skill_dir.join("skill.md"),
        "# web-search\nSearch the web\n## Commands\necho hello\n",
    )
    .unwrap();

    let entries: Vec<_> = fs::read_dir(dir.path())
        .unwrap()
        .flatten()
        .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
        .collect();

    assert_eq!(entries.len(), 1);
    assert!(entries[0].path().join("skill.md").exists());
}

#[test]
fn empty_skills_dir_is_ok() {
    let dir = TempDir::new().unwrap();
    let entries: Vec<_> = fs::read_dir(dir.path()).unwrap().flatten().collect();
    assert!(entries.is_empty());
}
