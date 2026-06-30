# Task 007 — Skills system (loader + executor)

## Goal
Implement the generic skill loader and executor that reads `skill.md` files and runs their commands.

## Prerequisites
- [x] task-006 (orchestrator, so skills can be invoked)

## Tasks
- [x] skills: Skill registry + loader (scan `skills/` dir, parse md) — `application/src/skills/mod.rs`
- [x] skills: Markdown parser (extract commands, prompts, metadata from skill.md) — `application/src/skills/loader.rs`
- [x] skills: Generic executor (run shell commands from skill.md, capture output) — `application/src/skills/executor.rs`
- [x] orchestrator: Hook skills into orchestrator (planner can request skill invocation) — `application/src/orchestrator/planner.rs`
- [x] test: Loader parses sample skill.md correctly — `application/tests/skill_loader_test.rs`
- [x] test: Executor runs commands and returns output — `application/tests/skill_executor_test.rs`

## Done When
- Skills discovered from `skills/` directory at startup
- Skill.md parsed into structured format (name, description, commands, prompt)
- Executor runs commands securely, captures stdout/stderr
- Orchestrator can invoke skills by name
