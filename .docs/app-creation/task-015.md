# Task 015 — esbuild Bundling & CLI Binary

## Goal
Configure esbuild to bundle the application into a single distributable JS file and create a `companion` CLI binary.

## Prerequisites
- [ ] task-014 (all modules integrated)

## Tasks
- [ ] build: Create esbuild config (bundle src/index.ts → dist/companion.js, ESM, Node target, externals) — `application/esbuild.config.ts`
- [ ] build: Add shebang injection (#!/usr/bin/env node) for CLI binary — `application/esbuild.config.ts`
- [ ] build: Configure package.json `bin` field for `companion` command — `application/package.json`
- [ ] build: Add `build` script to package.json — `application/package.json`
- [ ] test: `npm run build` produces dist/companion.js without errors — `application/tests/build.test.ts`
- [ ] test: Built binary executes and shows splash screen — `application/tests/build.test.ts`

## Done When
- `npm run build` produces `dist/companion.js` (single bundled file)
- `node dist/companion.js` starts the interactive CLI
- `npx companion` works after local install (bin field)
- Bundle size < 5MB (excluding node_modules)
- Build time < 2s
- All tests pass
