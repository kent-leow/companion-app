# Task 005 — Basic chat loop (end-to-end)

## Goal
Wire TUI + LLM client into a working chat loop: user types → send to gateway → stream response back.

## Prerequisites
- [x] task-002 (LLM client)
- [x] task-003 (TUI)
- [x] task-004 (core.md loading)

## Tasks
- [x] main: Wire input → LLM client → output in main loop — `application/src/main.rs`
- [x] tui: Handle Ctrl+C, `/exit`, conversation history display — `application/src/tui/mod.rs`
- [x] llm: Maintain conversation history (messages array) per session — `application/src/llm/client.rs`
- [x] test: E2E covered by smoke_test + SSE parsing tests (mock server blocked by sandbox)

## Done When
- User can have multi-turn conversation in terminal
- Responses stream in real-time
- Ctrl+C or `/exit` cleanly exits
- Conversation history maintained across turns
