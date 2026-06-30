# Companion App

Terminal-based AI agent CLI powered by Claude (Platform AI gateway). Orchestrator pattern with dynamic sub-agents, skills system, and self-evolving memory.

## Quick Start

```bash
cd application
cargo build --release
./target/release/companion
```

Requires env vars (already configured via `gt tools configure claude-code`):
- `ANTHROPIC_BASE_URL` — Platform AI gateway endpoint
- `ANTHROPIC_AUTH_TOKEN` — API key

## Features

- Interactive TUI with streaming responses
- Orchestrator decomposes complex queries into concurrent sub-agents
- Smart model routing: Haiku (simple) → Sonnet (standard) → Opus (complex)
- Skills loaded from `skills/<name>/skill.md` (no recompile to add)
- Local memory (8K token budget, self-pruning)
- Image input support (multimodal via `@image.png`)
- File/folder context injection (`@src/main.rs`)
- Multi-session isolation (for future Slack/Telegram integration)

## Structure

```
application/    — Rust CLI (the product)
landing/        — Landing/download website
.docs/          — Plans and task files
```

## Commands

| Action | Command | CWD |
|--------|---------|-----|
| Dev build | `cargo build` | `application/` |
| Release | `cargo build --release` | `application/` |
| Test | `cargo test` | `application/` |
| Run | `cargo run` | `application/` |

## Adding Skills

Drop a folder in `application/skills/<name>/skill.md`:

```markdown
# skill-name

Description of what this skill does.

## Commands

curl -s "https://example.com/?q={query}" | grep results

## Prompt

You are a specialist. Given the command output, summarize concisely.
```

No code changes needed — skills are discovered at startup.
