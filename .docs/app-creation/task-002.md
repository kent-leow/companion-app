# Task 002 — LLM client (Platform AI gateway)

## Goal
Implement the HTTP client that calls the Platform AI gateway (OpenAI-compatible) with streaming SSE support.

## Prerequisites
- [x] task-001 (project scaffold)

## Tasks
- [x] llm: Create LLM client abstraction — `application/src/llm/mod.rs`
- [x] llm: Implement OpenAI-compatible HTTP client with SSE streaming — `application/src/llm/client.rs`
- [x] llm: Model selector (Haiku/Sonnet/Opus routing logic) — `application/src/llm/model_selector.rs`
- [x] config: Load `ANTHROPIC_BASE_URL` + `ANTHROPIC_AUTH_TOKEN` from env — `application/src/config/env.rs`
- [x] test: Unit test model selector logic — `application/tests/model_selector_test.rs`
- [x] test: SSE parsing tests for streaming client — `application/tests/client_test.rs`

## Done When
- Can send a message to Platform AI gateway and receive streamed response
- Model selector returns correct model ID based on complexity hint
- Tests pass with mock HTTP server
