# Task 002 — LLM Client

## Goal
Implement the OpenAI-compatible API client for Platform AI gateway with SSE streaming and model selection.

## Prerequisites
- [ ] task-001 (config/env loaded)

## Tasks
- [ ] llm: Create LLM client abstraction with `chat()` method — `application/src/llm/index.ts`
- [ ] llm: Implement OpenAI-compatible HTTP client (POST /v1/chat/completions, x-api-key header) — `application/src/llm/client.ts`
- [ ] llm: Implement SSE stream parser (ReadableStream → token-by-token async iterable) — `application/src/llm/streaming.ts`
- [ ] llm: Implement model selector (Haiku for simple, Sonnet default, Opus for complex) — `application/src/llm/model-selector.ts`
- [ ] test: Client sends correct headers/body format — `application/tests/client.test.ts`
- [ ] test: Stream parser yields tokens from SSE chunks — `application/tests/streaming.test.ts`
- [ ] test: Model selector picks correct model by complexity — `application/tests/model-selector.test.ts`

## Done When
- `client.chat(messages, {stream: true})` returns async iterable of tokens
- `client.chat(messages, {stream: false})` returns full response string
- Model selector returns `bedrock.claude-haiku-4-5` / `bedrock.claude-sonnet-4-6` / `bedrock.claude-opus-4-6` based on input classification
- All tests pass
