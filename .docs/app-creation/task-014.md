# Task 014 — Integration & Entry Point

## Goal
Wire all modules together: CLI entry point with session init, @-tag file injection, image paste support, and the full interactive prompt loop.

## Prerequisites
- [ ] task-007 (TUI)
- [ ] task-008 (orchestrator)
- [ ] task-009 (sub-agents)
- [ ] task-010 (memory)
- [ ] task-011 (sessions)
- [ ] task-012 (default skills)

## Tasks
- [ ] entry: Wire CLI entry point — init config, load skills, create session, show splash, start prompt loop — `application/src/index.ts`
- [ ] input: Implement @-tag file content injection (detect @path → read file → append to user message) — `application/src/tui/input.ts`
- [ ] input: Implement image paste detection (base64 encode → multimodal message content part) — `application/src/tui/input.ts`
- [ ] integration: Connect TUI input → orchestrator.run() → TUI streaming output — `application/src/index.ts`
- [ ] integration: Wire memory load at session start, update after each turn — `application/src/index.ts`
- [ ] test: @-tag extracts path and injects file content into message — `application/tests/integration.test.ts`
- [ ] test: Full turn: user input → orchestrator → streamed response displayed — `application/tests/integration.test.ts`

## Done When
- `npx tsx src/index.ts` starts full interactive session (splash → prompt → response)
- @src/file.ts in input reads file and appends content to LLM context
- Image paste (if terminal supports) encodes as base64 multimodal content
- Memory persists between turns within a session
- Startup time < 500ms
- All tests pass
