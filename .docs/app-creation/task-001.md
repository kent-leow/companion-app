# Task 001 — Project Scaffolding & Config

## Goal
Set up the TypeScript project structure, package.json, tsconfig, and environment config loading.

## Prerequisites
- None (first task)

## Tasks
- [ ] scaffold: Initialize `package.json` with name, scripts (dev, build, start, test) — `application/package.json`
- [ ] scaffold: Create TypeScript config with strict mode, ESM output — `application/tsconfig.json`
- [ ] config: Implement env loader (ANTHROPIC_BASE_URL, ANTHROPIC_AUTH_TOKEN) — `application/src/config/env.ts`
- [ ] config: Implement config index (load env + core.md content) — `application/src/config/index.ts`
- [ ] scaffold: Create core.md system prompt template with action protocol instructions — `application/core.md`
- [ ] scaffold: Create entry point stub — `application/src/index.ts`
- [ ] test: Env loader validates required vars and returns typed config — `application/tests/config.test.ts`

## Done When
- `npm install` succeeds in `application/`
- `npx tsx src/index.ts` runs without error (prints "Companion ready" or similar)
- Tests pass: env loader throws on missing vars, returns correct shape when present
- `core.md` contains full action protocol (TOOL, SUBAGENT, RESPOND, THINK prefixes)
