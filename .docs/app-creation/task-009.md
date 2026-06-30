# Task 009 — Memory system (local, self-pruning)

## Goal
Implement local markdown-based memory with 8K token budget and self-pruning.

## Prerequisites
- [x] task-005 (basic chat loop, so memory can be used in context)

## Tasks
- [x] memory: Memory manager (load, save, query) — `application/src/memory/mod.rs`
- [x] memory: Markdown file store (read/write memory entries) — `application/src/memory/store.rs`
- [x] memory: Token-aware pruner (tiktoken-rs, 8K cap, importance scoring) — `application/src/memory/pruner.rs`
- [x] main: Load memory at session start, inject into context — `application/src/main.rs`
- [x] main: Update memory during session (LLM can request memory writes) — `application/src/main.rs`
- [x] test: Pruner respects 8K token limit — `application/tests/pruner_test.rs`
- [x] test: Memory persists across sessions — `application/tests/memory_store_test.rs`

## Done When
- Memory loaded at startup and injected into LLM context
- New facts stored during conversation
- Pruner keeps memory under 8K tokens (drops lowest-importance entries)
- Memory persists to disk between sessions
