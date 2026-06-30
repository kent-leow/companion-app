# Task 014 — Polish + performance optimization

## Goal
Final polish: ensure startup < 200ms, responses are ultra-concise, error handling is clean.

## Prerequisites
- [x] task-005 (chat loop)
- [x] task-006 (orchestrator)
- [x] task-009 (memory)

## Tasks
- [x] perf: Profile startup time, optimize cold start (lazy loading, parallel init) — `application/src/main.rs`
- [x] output: Enforce concise output formatting (≤3 sentences by default via core.md) — `application/core.md`
- [x] error: Graceful error handling (network failures, rate limits, invalid input) — `application/src/llm/client.rs`
- [x] tui: Loading indicators, typing animation during LLM response — `application/src/tui/output.rs`
- [x] test: Startup benchmark < 200ms — `application/tests/perf_test.rs`

## Done When
- `cargo build --release` binary starts in < 200ms
- All errors show user-friendly messages (no panics)
- Rate limit (429) triggers exponential backoff
- Loading state visible during LLM calls
