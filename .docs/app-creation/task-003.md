# Task 003 — Action Parser

## Goal
Implement the structured action prefix parser that extracts [TOOL], [SUBAGENT], [RESPOND], and [THINK] actions from LLM output.

## Prerequisites
- [ ] task-001 (project scaffolding)

## Tasks
- [ ] parser: Define ParsedAction interface (type, target, params, content) — `application/src/orchestrator/action-parser.ts`
- [ ] parser: Implement `parseActions(llmOutput: string): ParsedAction[]` — handles multi-line output, JSON param extraction — `application/src/orchestrator/action-parser.ts`
- [ ] parser: Handle edge cases — malformed JSON, missing prefixes, multi-line RESPOND content — `application/src/orchestrator/action-parser.ts`
- [ ] test: Parses single [TOOL:name] with JSON params — `application/tests/action-parser.test.ts`
- [ ] test: Parses multiple [SUBAGENT:role] lines — `application/tests/action-parser.test.ts`
- [ ] test: Parses [RESPOND] with multi-line content — `application/tests/action-parser.test.ts`
- [ ] test: Handles malformed input gracefully (no crash) — `application/tests/action-parser.test.ts`

## Done When
- `parseActions('[TOOL:web-search] {"query": "test"}')` returns `[{type: 'TOOL', target: 'web-search', params: {query: 'test'}}]`
- `parseActions('[RESPOND] Hello world')` returns `[{type: 'RESPOND', content: 'Hello world'}]`
- Mixed multi-line input correctly parsed into ordered action array
- All tests pass
