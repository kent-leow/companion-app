# Task 010 — Multi-session support

## Goal
Implement session isolation so multiple concurrent sessions can run (for future Slack/Telegram integration).

## Prerequisites
- [x] task-005 (basic chat loop)
- [x] task-009 (memory system)

## Tasks
- [x] session: Session manager (create, list, resume, destroy) — `application/src/session/mod.rs`
- [x] session: Per-session context isolation (history, memory, state) — `application/src/session/context.rs`
- [x] main: CLI flag `--session-id` for explicit session targeting — `application/src/main.rs`
- [x] test: Multiple sessions maintain isolated state — `application/tests/session_test.rs`

## Done When
- Each session has unique ID
- Conversation history isolated per session
- Memory can be per-session or shared (configurable)
- Sessions persist to disk (resumable)
