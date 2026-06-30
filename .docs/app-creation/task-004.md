# Task 004 — Core.md + config loading

## Goal
Implement `core.md` loading at session start and injection as system context into all LLM calls.

## Prerequisites
- [x] task-002 (LLM client)

## Tasks
- [x] config: core.md loader (read from disk, validate exists) — `application/src/config/mod.rs`
- [x] content: Create initial core.md with general-purpose agent instructions — `application/core.md`
- [x] llm: Inject core.md content as system message in every LLM call — `application/src/llm/client.rs`
- [x] test: Verify core.md is loaded and present in request payload — `application/tests/config_test.rs`

## Done When
- `core.md` is read at startup
- Every LLM request includes core.md as system message
- Missing core.md produces clear error
