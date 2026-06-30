## Agent README

Monorepo with two packages:

### Structure

- `application/` — Rust CLI agent (the product). Has its own `Cargo.toml`, build tooling. Run all cargo commands from this directory.
- `landing/` — Landing/download website.

### Quick Reference

| Action | Command | CWD |
|--------|---------|-----|
| Install deps | `cargo build` | `application/` |
| Test | `cargo test` | `application/` |
| Dev | `cargo run` | `application/` |
| Release build | `cargo build --release` | `application/` |

### Key Points

- All app source lives in `application/src/`.
- Tests use `cargo test` (standard Rust test framework).
- For detailed app architecture see `application/AGENTREADME.md`.
- Env vars required: `ANTHROPIC_BASE_URL`, `ANTHROPIC_AUTH_TOKEN` (set via `gt tools configure claude-code`).
- Skills are in `application/skills/<name>/skill.md` — no Rust code needed to add skills.
- SSL cert config in `~/.cargo/config.toml` (Cloudflare WARP proxy).
