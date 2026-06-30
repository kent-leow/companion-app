# Task 012 — File/folder @-tag context injection

## Goal
When user tags a file/folder path with `@`, read its content and inject into the LLM context.

## Prerequisites
- [x] task-003 (TUI input parsing)
- [x] task-005 (chat loop)

## Tasks
- [x] tui: Parse @-tags from input (file paths, folder paths) — `application/src/tui/input.rs`
- [x] context: Read file content for @-tagged paths, handle folders (list files) — `application/src/session/context.rs`
- [x] llm: Inject file content into user message before sending — `application/src/llm/client.rs`
- [x] test: @-tag parsing extracts correct paths — `application/tests/tag_parser_test.rs`
- [x] test: File content injection into message — `application/tests/context_inject_test.rs`

## Done When
- `@src/main.rs` injects file content into context
- `@src/` lists directory contents
- Non-existent paths produce clear error
- Large files are truncated with warning
