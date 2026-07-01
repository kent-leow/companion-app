# Task 005 — Skill Executor & Matcher

## Goal
Implement skill command execution (child_process spawn with timeout) and intent-to-skill matching via trigger keywords.

## Prerequisites
- [ ] task-004 (skill schema, loader, registry)

## Tasks
- [ ] executor: Implement command template interpolation (replace `{param}` placeholders) — `application/src/skills/executor.ts`
- [ ] executor: Implement `executeSkill(skill, params)` — spawn shell command, capture stdout/stderr, enforce timeout — `application/src/skills/executor.ts`
- [ ] executor: Return structured result (stdout, stderr, exitCode, timedOut) — `application/src/skills/executor.ts`
- [ ] matcher: Implement `matchSkill(query, registry)` — keyword matching against triggers + description — `application/src/skills/matcher.ts`
- [ ] test: Executor interpolates params into template correctly — `application/tests/skill-executor.test.ts`
- [ ] test: Executor kills process on timeout — `application/tests/skill-executor.test.ts`
- [ ] test: Matcher returns correct skill for trigger keywords — `application/tests/skill-matcher.test.ts`
- [ ] test: Matcher returns null for unmatched queries — `application/tests/skill-matcher.test.ts`

## Done When
- `executeSkill(skill, {query: "test"})` spawns command with substituted params
- Timed-out commands return `{timedOut: true}` within configured timeout + 1s
- `matchSkill("search the web for X", registry)` returns web-search skill
- `matchSkill("random unrelated input", registry)` returns null
- All tests pass
