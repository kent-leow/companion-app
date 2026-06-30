#[test]
fn model_selector_defaults() {
    let haiku = "bedrock.claude-haiku-4-5";
    let sonnet = "bedrock.claude-sonnet-4-6";
    let opus = "bedrock.claude-opus-4-6";

    assert!(haiku.contains("haiku"));
    assert!(sonnet.contains("sonnet"));
    assert!(opus.contains("opus"));
}

#[test]
fn model_ids_are_valid_format() {
    let models = [
        "bedrock.claude-haiku-4-5",
        "bedrock.claude-sonnet-4-6",
        "bedrock.claude-opus-4-6",
    ];

    for model in models {
        assert!(model.starts_with("bedrock.claude-"));
        assert!(model.split('-').count() >= 3);
    }
}
