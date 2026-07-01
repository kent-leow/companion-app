# Task 011 — Session Management

## Goal
Implement multi-session support with isolated state (conversation, memory, config) per session ID.

## Prerequisites
- [ ] task-006 (context manager)
- [ ] task-010 (memory system)

## Tasks
- [ ] session: Define Session interface (id, contextManager, memory, config, createdAt) — `application/src/session/index.ts`
- [ ] session: Implement session context (per-session state isolation, shared config) — `application/src/session/context.ts`
- [ ] session: Implement session manager (create, get, list, destroy sessions) — `application/src/session/index.ts`
- [ ] session: Generate unique session IDs — `application/src/session/index.ts`
- [ ] test: Creating session initializes isolated state — `application/tests/session.test.ts`
- [ ] test: Multiple sessions maintain independent conversation context — `application/tests/session.test.ts`
- [ ] test: Sessions share config but isolate memory and context — `application/tests/session.test.ts`

## Done When
- `sessionManager.create()` returns new session with unique ID
- Each session has its own context manager and memory instance
- Sessions share global config (env vars, skill registry)
- Session state persists within process lifetime
- All tests pass
