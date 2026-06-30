# Task 001 — Project scaffold + Rust boilerplate

## Goal
Set up the Rust project with Cargo, dependencies, and a minimal "hello world" binary that compiles and runs.

## Prerequisites
- None (first task)

## Tasks
- [x] init: `cargo init` in `application/` — `application/Cargo.toml`
- [x] deps: Add tokio, reqwest, serde, serde_json, clap, crossterm, ratatui — `application/Cargo.toml`
- [x] entry: Minimal main.rs with clap CLI setup + tokio runtime — `application/src/main.rs`
- [x] config: Module stubs for config/mod.rs, config/env.rs — `application/src/config/mod.rs`, `application/src/config/env.rs`
- [x] test: Verify binary compiles and runs — `application/tests/smoke_test.rs`

## Done When
- `cargo build --release` succeeds
- `cargo run` prints version/help
- CI-ready: `cargo test` passes
