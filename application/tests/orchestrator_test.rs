#[test]
fn simple_query_detection() {
    fn is_simple(input: &str) -> bool {
        let word_count = input.split_whitespace().count();
        word_count <= 10 && !input.contains("and") && !input.contains("then")
    }

    assert!(is_simple("what is rust"));
    assert!(is_simple("hello"));
    assert!(!is_simple("search for rust tutorials and summarize them"));
    assert!(!is_simple("first do this then do that"));
    assert!(!is_simple(
        "explain the architecture of this project in detail with examples and comparisons"
    ));
}

#[test]
fn prompt_builder_includes_core_and_role() {
    let core = "Be concise.";
    let role = "web researcher";
    let task = "find Rust crates";

    let system_prompt = format!(
        "{}\n\n---\nYour role: {}\nRespond concisely. No preambles.",
        core, role
    );

    assert!(system_prompt.contains("Be concise."));
    assert!(system_prompt.contains("web researcher"));
    assert!(!system_prompt.contains(task));
}
