# Task 004 — Skill Schema, Loader & Registry

## Goal
Implement the skill system: YAML frontmatter schema types, skill.md parser, and in-memory skill registry with trigger index.

## Prerequisites
- [ ] task-001 (project scaffolding, package.json for `yaml` dep)

## Tasks
- [ ] skills: Define SkillDef TypeScript interface (name, version, description, triggers, parameters, auth, commands) — `application/src/skills/schema.ts`
- [ ] skills: Implement skill.md YAML frontmatter parser → SkillDef — `application/src/skills/loader.ts`
- [ ] skills: Implement skill registry (scan directory, load all skills, build triggerMap) — `application/src/skills/index.ts`
- [ ] skills: Generate "Available Tools" section string from loaded skills — `application/src/skills/index.ts`
- [ ] test: Loader parses valid skill.md into correct SkillDef shape — `application/tests/skill-loader.test.ts`
- [ ] test: Registry builds triggerMap from multiple skills — `application/tests/skill-registry.test.ts`
- [ ] test: Registry generates Available Tools prompt section — `application/tests/skill-registry.test.ts`

## Done When
- `loadSkill('path/to/skill.md')` returns typed SkillDef with all frontmatter fields
- `SkillRegistry.loadAll('skills/')` populates skills map and triggerMap
- `registry.getToolsPromptSection()` returns formatted string listing all skills with descriptions and params
- Auth warnings logged for skills with missing env vars
- All tests pass
