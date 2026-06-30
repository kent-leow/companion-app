use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SubAgentSpec {
    role: String,
    task: String,
    model_hint: String,
}

fn extract_json(text: &str) -> &str {
    let trimmed = text.trim();
    if trimmed.starts_with('[') {
        if let Some(end) = trimmed.rfind(']') {
            return &trimmed[..=end];
        }
    }
    if let Some(start) = trimmed.find('[') {
        if let Some(end) = trimmed.rfind(']') {
            return &trimmed[start..=end];
        }
    }
    "[]"
}

#[test]
fn parses_valid_plan_json() {
    let response = r#"[{"role":"web researcher","task":"search for Rust TUI libraries","model_hint":"bedrock.claude-haiku-4-5"}]"#;
    let json_str = extract_json(response);
    let specs: Vec<SubAgentSpec> = serde_json::from_str(json_str).unwrap();
    assert_eq!(specs.len(), 1);
    assert_eq!(specs[0].role, "web researcher");
    assert!(specs[0].model_hint.contains("haiku"));
}

#[test]
fn parses_empty_plan() {
    let response = "[]";
    let json_str = extract_json(response);
    let specs: Vec<SubAgentSpec> = serde_json::from_str(json_str).unwrap();
    assert!(specs.is_empty());
}

#[test]
fn extracts_json_from_surrounding_text() {
    let response = r#"Here's the plan:
[{"role":"analyzer","task":"analyze code","model_hint":"bedrock.claude-sonnet-4-6"}]
That should work."#;
    let json_str = extract_json(response);
    let specs: Vec<SubAgentSpec> = serde_json::from_str(json_str).unwrap();
    assert_eq!(specs.len(), 1);
    assert_eq!(specs[0].role, "analyzer");
}

#[test]
fn handles_multiple_agents() {
    let response = r#"[
  {"role":"researcher","task":"find docs","model_hint":"bedrock.claude-haiku-4-5"},
  {"role":"coder","task":"write impl","model_hint":"bedrock.claude-sonnet-4-6"},
  {"role":"reviewer","task":"check quality","model_hint":"bedrock.claude-opus-4-6"}
]"#;
    let json_str = extract_json(response);
    let specs: Vec<SubAgentSpec> = serde_json::from_str(json_str).unwrap();
    assert_eq!(specs.len(), 3);
}

#[test]
fn invalid_json_returns_empty() {
    let response = "I can answer this directly without any agents.";
    let json_str = extract_json(response);
    let specs: Vec<SubAgentSpec> = serde_json::from_str(json_str).unwrap();
    assert!(specs.is_empty());
}
