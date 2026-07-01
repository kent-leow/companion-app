# Task 006 — Context Manager

## Goal
Implement the rolling conversation summary system that provides concise context to each LLM call without sending full history.

## Prerequisites
- [ ] task-002 (LLM client for summarization calls)

## Tasks
- [ ] context: Define ConversationSummary interface (summary, lastUserMessage, lastAssistantResponse, turnCount) — `application/src/orchestrator/context-manager.ts`
- [ ] context: Implement `updateSummary()` — if turnCount < 4: keep raw messages; if >= 4: LLM-summarize prior turns — `application/src/orchestrator/context-manager.ts`
- [ ] context: Implement `getContextForPrompt()` — returns formatted context string (≤2K tokens) — `application/src/orchestrator/context-manager.ts`
- [ ] context: Implement token counting for summary cap (using tiktoken or char approximation) — `application/src/orchestrator/context-manager.ts`
- [ ] test: First 3 turns return raw messages as context — `application/tests/context-manager.test.ts`
- [ ] test: Turn 4+ triggers summarization and caps at 2K tokens — `application/tests/context-manager.test.ts`
- [ ] test: getContextForPrompt returns formatted string with summary + last exchange — `application/tests/context-manager.test.ts`

## Done When
- Context manager tracks conversation turns and produces rolling summary
- After 4+ turns, summary is condensed via LLM call
- Context string never exceeds 2K tokens (~8000 chars)
- Sub-agents receive 50-200 token context summary, not full history
- All tests pass
