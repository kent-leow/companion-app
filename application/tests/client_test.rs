use serde_json;

#[derive(serde::Deserialize)]
struct ChatChunk {
    choices: Option<Vec<ChunkChoice>>,
}

#[derive(serde::Deserialize)]
struct ChunkChoice {
    delta: Option<Delta>,
    finish_reason: Option<String>,
}

#[derive(serde::Deserialize)]
struct Delta {
    content: Option<String>,
}

fn parse_sse_line(line: &str) -> Option<String> {
    let data = line.strip_prefix("data: ")?;
    if data == "[DONE]" {
        return None;
    }
    let chunk: ChatChunk = serde_json::from_str(data).ok()?;
    let choices = chunk.choices?;
    choices.into_iter().find_map(|c| c.delta?.content)
}

#[test]
fn parses_sse_token_chunk() {
    let line = r#"data: {"choices":[{"delta":{"content":"Hello"},"finish_reason":null}]}"#;
    assert_eq!(parse_sse_line(line), Some("Hello".to_string()));
}

#[test]
fn parses_sse_done_marker() {
    let line = "data: [DONE]";
    assert_eq!(parse_sse_line(line), None);
}

#[test]
fn parses_sse_finish_reason() {
    let line = r#"data: {"choices":[{"delta":{},"finish_reason":"stop"}]}"#;
    assert_eq!(parse_sse_line(line), None);
}

#[test]
fn parses_full_sse_stream() {
    let stream = "\
data: {\"choices\":[{\"delta\":{\"content\":\"Hello\"},\"finish_reason\":null}]}\n\
data: {\"choices\":[{\"delta\":{\"content\":\" world\"},\"finish_reason\":null}]}\n\
data: {\"choices\":[{\"delta\":{},\"finish_reason\":\"stop\"}]}\n\
data: [DONE]\n";

    let tokens: Vec<String> = stream
        .lines()
        .filter(|l| !l.is_empty())
        .filter_map(|l| parse_sse_line(l))
        .collect();

    assert_eq!(tokens, vec!["Hello", " world"]);
}

#[test]
fn request_body_format_is_correct() {
    let body = serde_json::json!({
        "model": "bedrock.claude-sonnet-4-6",
        "messages": [
            {"role": "system", "content": "You are helpful."},
            {"role": "user", "content": "hi"}
        ],
        "stream": true,
        "max_tokens": 4096
    });

    assert_eq!(body["model"], "bedrock.claude-sonnet-4-6");
    assert_eq!(body["messages"][0]["role"], "system");
    assert_eq!(body["stream"], true);
    assert_eq!(body["max_tokens"], 4096);
}
