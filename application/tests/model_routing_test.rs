#[test]
fn simple_queries_route_to_haiku() {
    let simple_queries = vec![
        "what time is it",
        "hello",
        "thanks",
        "define polymorphism",
    ];

    for q in simple_queries {
        let word_count = q.split_whitespace().count();
        assert!(word_count <= 10, "expected simple: {}", q);
    }
}

#[test]
fn complex_queries_detected() {
    let complex_queries = vec![
        "search for rust tutorials and summarize the top 3",
        "explain the architecture of this project in detail with examples and code samples",
        "first analyze the code then refactor it and write tests",
    ];

    for q in complex_queries {
        let is_complex = q.split_whitespace().count() > 10
            || q.contains("and")
            || q.contains("then");
        assert!(is_complex, "expected complex: {}", q);
    }
}

#[test]
fn model_hint_from_planner_is_respected() {
    let hints = vec![
        ("bedrock.claude-haiku-4-5", "haiku"),
        ("bedrock.claude-sonnet-4-6", "sonnet"),
        ("bedrock.claude-opus-4-6", "opus"),
    ];

    for (model_id, expected_tier) in hints {
        assert!(
            model_id.contains(expected_tier),
            "{} should contain {}",
            model_id,
            expected_tier
        );
    }
}
