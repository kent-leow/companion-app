# Task 006 — Orchestrator + dynamic sub-agent spawning

## Goal
Implement the orchestrator that decomposes complex queries into sub-tasks and dynamically spawns role-prompted sub-agents.

## Prerequisites
- [x] task-002 (LLM client)
- [x] task-004 (core.md loading)

## Tasks
- [x] orchestrator: Core orchestrator logic (detect simple vs complex query) — `application/src/orchestrator/mod.rs`
- [x] orchestrator: Planner — LLM call that outputs `[{role, task, model_hint}]` JSON — `application/src/orchestrator/planner.rs`
- [x] orchestrator: Synthesizer — merge sub-agent outputs into concise response — `application/src/orchestrator/synthesizer.rs`
- [x] agent: Dynamic sub-agent spawner (tokio tasks) — `application/src/agent/mod.rs`
- [x] agent: Async agent pool with timeout/cancellation — `application/src/agent/pool.rs`
- [x] agent: Prompt builder (core.md + role prompt construction) — `application/src/agent/prompt_builder.rs`
- [x] test: Planner outputs valid JSON sub-task array — `application/tests/planner_test.rs`
- [x] test: Pool spawns N agents concurrently and collects results — `application/tests/pool_test.rs`
- [x] test: Orchestrator end-to-end (plan → spawn → synthesize) — `application/tests/orchestrator_test.rs`

## Done When
- Simple queries bypass orchestration (direct to LLM)
- Complex queries decompose into sub-tasks
- Sub-agents spawn concurrently with role prompts + core.md
- Results are synthesized into single concise answer
- Timeout kills stalled sub-agents
