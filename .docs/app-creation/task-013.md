# Task 013 — Smart model routing integration

## Goal
Wire the model selector into the orchestrator so it automatically picks Haiku/Sonnet/Opus based on task complexity.

## Prerequisites
- [x] task-006 (orchestrator)
- [x] task-002 (model selector)

## Tasks
- [x] orchestrator: Pass complexity assessment to model selector — `application/src/orchestrator/mod.rs`
- [x] llm: Model selector uses heuristics (token count, keyword detection, planner hint) — `application/src/llm/model_selector.rs`
- [x] agent: Sub-agents use model_hint from planner output — `application/src/agent/mod.rs`
- [x] test: Routing logic selects expected model per scenario — `application/tests/model_routing_test.rs`

## Done When
- Simple "what time is it" → Haiku
- Standard coding question → Sonnet
- Complex multi-step reasoning → Opus
- Sub-agents respect planner's model_hint
