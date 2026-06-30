# Task 003 — Interactive TUI + splash screen

## Goal
Build the terminal UI: alien pixel art splash, input prompt, and streaming output rendering.

## Prerequisites
- [x] task-001 (project scaffold)

## Tasks
- [x] tui: Module setup — `application/src/tui/mod.rs`
- [x] tui: Input handler (text, multiline, @-tag detection, image paste) — `application/src/tui/input.rs`
- [x] tui: Streaming output renderer (token-by-token display) — `application/src/tui/output.rs`
- [x] tui: Alien pixel art splash screen on startup — `application/src/tui/splash.rs`
- [x] test: Input parser tests (@-tag extraction, image detection) — `application/tests/input_test.rs`

## Done When
- Binary starts with alien pixel art splash
- User can type input and see it echoed
- @-tags are detected and highlighted
- Streaming text renders token-by-token
