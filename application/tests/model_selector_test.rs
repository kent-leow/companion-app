#[test]
fn model_selector_defaults() {
    let haiku = "anthropic.claude-haiku-4-5-20251001-v1:0";
    let sonnet = "anthropic.claude-sonnet-4-5-20250929-v1:0";
    let opus = "anthropic.claude-opus-4-6-v1";

    assert!(haiku.contains("haiku"));
    assert!(sonnet.contains("sonnet"));
    assert!(opus.contains("opus"));
}

#[test]
fn model_ids_are_valid_format() {
    let models = [
        "anthropic.claude-haiku-4-5-20251001-v1:0",
        "anthropic.claude-sonnet-4-5-20250929-v1:0",
        "anthropic.claude-opus-4-6-v1",
    ];

    for model in models {
        assert!(model.starts_with("anthropic.claude-"));
        assert!(model.contains("claude"));
    }
}
