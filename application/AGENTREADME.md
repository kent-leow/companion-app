## Application Architecture

Rust CLI agent with orchestrator pattern.

### Module Map

| Module | Purpose |
|--------|---------|
| `src/main.rs` | Entry, CLI args, chat loop |
| `src/llm/` | Platform AI gateway client (OpenAI-compatible SSE), model selector |
| `src/orchestrator/` | Planner (decompose query) + Synthesizer (merge results) |
| `src/agent/` | Dynamic sub-agent pool (tokio tasks, role prompts, timeout) |
| `src/skills/` | Skill loader (parse skill.md) + generic executor (shell commands) |
| `src/memory/` | Markdown store, 8K token pruner |
| `src/session/` | Multi-session manager, per-session context isolation |
| `src/tui/` | Input (@-tags, image detect), output (streaming), splash (alien art) |
| `src/config/` | Env vars + core.md loader |

### Data Flow

```
User Input → TUI (parse @-tags, detect images)
    → Orchestrator: simple? → direct LLM call
                   complex? → Planner → [sub-agents] → Synthesizer
    → TUI Output (stream tokens)
```

### Key Files

- `core.md` — System instructions injected into every LLM call
- `skills/web-search/skill.md` — DuckDuckGo search skill definition
- `Cargo.toml` — Dependencies (tokio, reqwest, serde, clap, crossterm, ratatui)

### LLM Integration

```
Endpoint: $ANTHROPIC_BASE_URL/v1/chat/completions
Auth:     x-api-key: $ANTHROPIC_AUTH_TOKEN
Format:   OpenAI-compatible (messages API)
Models:   bedrock.claude-haiku-4-5 | bedrock.claude-sonnet-4-6 | bedrock.claude-opus-4-6
```

### Testing

```bash
cargo test                    # All 57 tests
cargo test --test client_test # Specific test file
```

Tests are in `tests/` (integration) — no unit tests in src to keep modules clean.
