# Task 010 — Memory System

## Goal
Implement markdown-based memory store with read/write/prune operations and 8K token budget enforcement.

## Prerequisites
- [ ] task-001 (project scaffolding)

## Tasks
- [ ] memory: Implement memory store (read/write/append markdown file) — `application/src/memory/store.ts`
- [ ] memory: Implement 8K token pruner (count tokens, remove oldest entries when over budget) — `application/src/memory/pruner.ts`
- [ ] memory: Implement memory manager (load at session start, save during session, auto-prune) — `application/src/memory/index.ts`
- [ ] test: Store reads and writes markdown entries correctly — `application/tests/memory-store.test.ts`
- [ ] test: Pruner removes oldest entries when exceeding 8K tokens — `application/tests/memory-pruner.test.ts`
- [ ] test: Memory manager auto-prunes on write when budget exceeded — `application/tests/memory.test.ts`

## Done When
- `memory.read()` returns current memory contents as string
- `memory.append(entry)` adds timestamped entry to memory file
- `memory.prune()` removes oldest entries until under 8K tokens
- Memory file persists between sessions (markdown in project dir)
- All tests pass
