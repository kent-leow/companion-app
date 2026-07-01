# Task 007 — Terminal UI (Input, Output, Splash)

## Goal
Implement the interactive terminal interface: alien pixel art splash, user input handling (text + @-tags), and streaming markdown output rendering.

## Prerequisites
- [ ] task-001 (package.json for chalk, ora, marked-terminal deps)

## Tasks
- [ ] tui: Implement alien pixel art splash screen (ASCII/braille art via chalk) — `application/src/tui/splash.ts`
- [ ] tui: Implement input handler (readline-based, supports @-tag detection for file paths) — `application/src/tui/input.ts`
- [ ] tui: Implement streaming output renderer (markdown via marked-terminal, token-by-token display) — `application/src/tui/output.ts`
- [ ] tui: Create TUI module index (init, prompt loop, display) — `application/src/tui/index.ts`
- [ ] test: Input parser extracts @-tagged file paths from user text — `application/tests/tui-input.test.ts`
- [ ] test: Output renderer formats markdown correctly (code blocks, bullets) — `application/tests/tui-output.test.ts`

## Done When
- Running the app shows colored alien pixel art on startup
- User can type multi-line input with @path tags detected and extracted
- LLM streaming response renders token-by-token with markdown formatting
- Spinner shown during LLM processing
- All tests pass
