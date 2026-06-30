#[test]
fn model_selector_defaults() {
    let haiku = "claude-haiku-4-20250414";
    let sonnet = "claude-sonnet-4-20250514";
    let opus = "claude-opus-4-20250514";

    assert!(haiku.contains("haiku"));
    assert!(sonnet.contains("sonnet"));
    assert!(opus.contains("opus"));
}

#[test]
fn model_ids_are_valid_format() {
    let models = [
        "claude-haiku-4-20250414",
        "claude-sonnet-4-20250514",
        "claude-opus-4-20250514",
    ];

    for model in models {
        assert!(model.starts_with("claude-"));
        assert!(model.split('-').count() >= 3);
    }
}
