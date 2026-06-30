# Task 011 — Image paste support (multimodal)

## Goal
Enable image input in the terminal (paste or file path) sent as base64 multimodal message to Claude.

## Prerequisites
- [x] task-003 (TUI input handling)
- [x] task-002 (LLM client supports multimodal format)

## Tasks
- [x] tui: Detect image paste (iTerm2/Kitty protocol) or `@image.png` file reference — `application/src/tui/input.rs`
- [x] llm: Build multimodal message payload (image_url with base64 data URI) — `application/src/llm/client.rs`
- [x] test: Image file read + base64 encoding — `application/tests/image_input_test.rs`

## Done When
- User can reference an image file (`@screenshot.png`)
- Image is base64 encoded and sent in multimodal format
- Claude responds about the image content
