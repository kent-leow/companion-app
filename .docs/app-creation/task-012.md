# Task 012 — Default Skills (Zero-Config)

## Goal
Create the 5 default skill.md files that ship out-of-box with no API keys required: web-search, web-fetch, read-file, run-command, summarize.

## Prerequisites
- [ ] task-004 (skill schema/loader)
- [ ] task-005 (skill executor)

## Tasks
- [ ] skill: Create web-search skill.md (DuckDuckGo Lite, iterative via action loop) — `application/skills/web-search/skill.md`
- [ ] skill: Create web-fetch skill.md (curl + HTML strip + truncate to 6000 chars) — `application/skills/web-fetch/skill.md`
- [ ] skill: Create read-file skill.md (cat/head with line limit) — `application/skills/read-file/skill.md`
- [ ] skill: Create run-command skill.md (shell exec with 30s timeout, 100-line cap) — `application/skills/run-command/skill.md`
- [ ] skill: Create summarize skill.md (LLM-only, no command, bullet-point output) — `application/skills/summarize/skill.md`
- [ ] test: All 5 skills load via registry without errors — `application/tests/default-skills.test.ts`
- [ ] test: web-search command template produces valid curl command — `application/tests/default-skills.test.ts`
- [ ] test: Skill executor runs read-file skill successfully on a test file — `application/tests/default-skills.test.ts`

## Done When
- `SkillRegistry.loadAll('skills/')` loads all 5 default skills
- Each skill has valid YAML frontmatter matching SkillDef schema
- web-search curl command returns DuckDuckGo results when executed
- read-file skill reads actual files correctly
- run-command skill executes `echo hello` and captures output
- All tests pass
