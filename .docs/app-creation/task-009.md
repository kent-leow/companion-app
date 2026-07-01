# Task 009 — Sub-Agent System

## Goal
Implement sub-agent spawning: prompt builder with role/context injection, concurrent execution pool, and result collection.

## Prerequisites
- [ ] task-002 (LLM client)
- [ ] task-003 (action parser)
- [ ] task-006 (context manager for summary injection)

## Tasks
- [ ] agent: Define SubAgentSpec interface (role, task, modelHint, skill, contextNeeded) — `application/src/agent/index.ts`
- [ ] agent: Implement prompt builder (sub-agent core.md variant + role + task + context summary + tools subset) — `application/src/agent/prompt-builder.ts`
- [ ] agent: Implement agent pool (Promise.all with per-agent timeout, collect results) — `application/src/agent/pool.ts`
- [ ] agent: Implement sub-agent execution (own action loop — TOOL + RESPOND only, no recursive SUBAGENT) — `application/src/agent/index.ts`
- [ ] test: Prompt builder produces correct system/user messages with role and context — `application/tests/agent-prompt.test.ts`
- [ ] test: Pool runs agents concurrently and collects results — `application/tests/agent-pool.test.ts`
- [ ] test: Sub-agent cannot spawn nested sub-agents — `application/tests/agent-pool.test.ts`

## Done When
- `[SUBAGENT:researcher] {"task": "Find X"}` parsed by orchestrator → spawns sub-agent with focused prompt
- Multiple sub-agents run via Promise.all, results collected
- Each sub-agent gets: sub-agent core.md + role + task + 50-200 token context summary
- Sub-agents can use [TOOL] but NOT [SUBAGENT]
- Timed-out agents return partial result or error
- All tests pass
