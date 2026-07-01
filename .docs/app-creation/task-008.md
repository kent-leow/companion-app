# Task 008 — Orchestrator Core Loop

## Goal
Implement the main agentic loop: build system prompt → call LLM → parse actions → execute tools → loop until [RESPOND].

## Prerequisites
- [ ] task-002 (LLM client)
- [ ] task-003 (action parser)
- [ ] task-004 (skill registry)
- [ ] task-005 (skill executor)
- [ ] task-006 (context manager)

## Tasks
- [ ] orchestrator: Implement system prompt builder (core.md + available tools section + context summary) — `application/src/orchestrator/index.ts`
- [ ] orchestrator: Implement main `agentLoop(userMessage)` — calls LLM, parses actions, executes tools, loops until RESPOND — `application/src/orchestrator/index.ts`
- [ ] orchestrator: Wire TOOL action → skill executor → feed TOOL_RESULT back to LLM — `application/src/orchestrator/index.ts`
- [ ] orchestrator: Implement 10-iteration safety cap — `application/src/orchestrator/index.ts`
- [ ] orchestrator: Implement malformed prefix retry (max 2 retries) — `application/src/orchestrator/index.ts`
- [ ] test: Loop terminates on [RESPOND] and returns content — `application/tests/orchestrator.test.ts`
- [ ] test: TOOL action triggers skill execution and feeds result back — `application/tests/orchestrator.test.ts`
- [ ] test: Safety cap prevents infinite loops — `application/tests/orchestrator.test.ts`

## Done When
- `orchestrator.run("What is 2+2?")` → LLM emits `[RESPOND] 4` → returns "4"
- `orchestrator.run("Search for X")` → LLM emits `[TOOL:web-search]` → executes → feeds result → LLM emits `[RESPOND]` → returns answer
- Loop stops after 10 iterations with error message
- Context manager updated after each completed turn
- All tests pass
