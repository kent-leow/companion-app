# Plan: Companion Agent App

## Summary

Build a terminal-based AI agent CLI (like Claude Code / GitHub Copilot CLI) in **Rust** that calls Claude models via **Platform AI gateway** (Anthropic-compatible Messages API). The orchestrator manages core context (`core.md`), dynamically spins up sub-agents with role-based prompts, collects responses async, then synthesizes a concise answer. Sub-agents are not hardcoded — they are created on-the-fly based on task decomposition. Skills loaded from markdown. Memory is local and self-evolving (8K token cap). Supports concurrent sessions for future chat integrations.

## Scope

### In

- Rust CLI binary with attractive interactive TUI (alien pixel art branding)
- Platform AI gateway integration (Anthropic Messages API format, streaming)
- Smart model switching: Haiku (simple) → Sonnet (default) → Opus (complex)
- Orchestrator → dynamic sub-agent fan-out (role-based prompts, no hardcoded agent types)
- `core.md` loaded every session start as system context (general-purpose instructions)
- Skills system reading from `skills/<name>/skill.md`
- Web-search skill (DuckDuckGo, free/open-source) as first default skill
- Local memory store (markdown-based, 8K token budget, self-pruning)
- Interactive input: paste text, paste images (Claude multimodal), tag file/folder paths
- Multi-session support (session isolation for future Slack/Telegram integrations)
- Ultra-concise professional output formatting

### Out

- Electron desktop wrapper (separate effort, later)
- Telegram/Slack bot integrations (architecture supports it, implementation later)
- Landing page (separate package, already scoped)

## Acceptance Criteria

| # | Criteria |
|---|----------|
| 1 | `companion` binary starts interactive terminal with alien pixel art splash + prompt |
| 2 | User input sent to Platform AI gateway (Sonnet default), streamed response displayed |
| 3 | Orchestrator decomposes multi-step queries into sub-tasks, dynamically creates role-prompted sub-agents |
| 4 | Sub-agents run concurrently, results collected and synthesized into single concise response |
| 5 | Model selection automatic: simple Q → Haiku, standard → Sonnet, complex reasoning → Opus |
| 6 | Sub-agents receive `core.md` context + role-specific prompt (no hardcoded agent logic) |
| 7 | Skills loaded from `skills/<name>/skill.md` and invocable by orchestrator |
| 8 | Web-search skill (DuckDuckGo) performs search and returns summarized results |
| 9 | `core.md` loaded at session start and injected as system context for all LLM calls |
| 10 | Memory file read at start, updated during session, pruned to stay under 8K tokens |
| 11 | File/folder paths tagged in input (`@src/main.rs`) → content injected into context |
| 12 | Image paste supported in terminal input (base64 encoded, sent as multimodal message) |
| 13 | Multiple concurrent sessions supported (session ID isolation, shared config) |
| 14 | Responses are professional, ultra-concise (≤3 sentences default) |
| 15 | Startup time < 200ms on M-series Mac |

## Open Questions

All resolved.

| # | Question | Resolution |
|---|----------|------------|
| ~~1~~ | AWS/auth credentials | Uses Platform AI gateway. Auth via `ANTHROPIC_BASE_URL` + `ANTHROPIC_AUTH_TOKEN` env vars (already in `~/.zshenv` via `gt tools configure claude-code`) |
| ~~2~~ | Bedrock region | N/A — routed through Platform AI gateway (`api.ai.tech.gov.sg`), no direct Bedrock calls |
| ~~3~~ | Memory token budget | 8K tokens |
| ~~4~~ | Web-search provider | DuckDuckGo (free, open-source, no API key needed) |

## Estimate

- AC rows: 15
- Open Questions: 0
- Raw: (15 × 2) + 0 = 30
- **Story Points: 21** (nearest Fibonacci)
- **~42 days** (1 SP = 2 days)

## Folder Structure

```
application/
├── Cargo.toml
├── core.md                    # System instructions (loaded every session)
├── src/
│   ├── main.rs                # Entry, CLI setup, session init
│   ├── tui/
│   │   ├── mod.rs             # Terminal UI module
│   │   ├── input.rs           # Input: text, @-tags, image paste
│   │   ├── output.rs          # Streaming response rendering
│   │   └── splash.rs          # Alien pixel art startup screen
│   ├── orchestrator/
│   │   ├── mod.rs             # Core orchestrator logic
│   │   ├── planner.rs         # Task decomposition → dynamic sub-agent specs
│   │   └── synthesizer.rs     # Response aggregation from sub-agents
│   ├── agent/
│   │   ├── mod.rs             # Dynamic sub-agent spawner
│   │   ├── pool.rs            # Async agent pool (tokio tasks)
│   │   └── prompt_builder.rs  # Role-based prompt construction + core.md injection
│   ├── llm/
│   │   ├── mod.rs             # LLM client abstraction
│   │   ├── client.rs          # Anthropic Messages API client (Platform AI gateway)
│   │   └── model_selector.rs  # Smart model routing (Haiku/Sonnet/Opus)
│   ├── skills/
│   │   ├── mod.rs             # Skill loader + registry
│   │   ├── loader.rs          # Markdown skill parser (extract commands + prompt)
│   │   └── executor.rs        # Generic: parse md → shell exec → return output
│   ├── memory/
│   │   ├── mod.rs             # Memory manager
│   │   ├── store.rs           # Read/write markdown memory
│   │   └── pruner.rs          # 8K token-aware pruning
│   ├── session/
│   │   ├── mod.rs             # Session manager (multi-session support)
│   │   └── context.rs         # Per-session context isolation
│   └── config/
│       ├── mod.rs             # Config loading (env vars, core.md)
│       └── env.rs             # Platform AI gateway config from env
├── skills/
│   └── web-search/
│       └── skill.md           # Web search skill definition
└── tests/
    ├── orchestrator_test.rs
    ├── client_test.rs
    └── memory_test.rs
```

## Tech Stack

| Component | Choice | Why |
|-----------|--------|-----|
| Language | Rust | Fast startup, zero-cost async, no GC, native binary |
| Async runtime | tokio | Industry standard, great for concurrent sub-agents |
| TUI | crossterm + ratatui | Cross-platform terminal, rich rendering, pixel art |
| HTTP client | reqwest | Anthropic API calls + DuckDuckGo search |
| Serialization | serde + serde_json | Standard for Rust |
| CLI args | clap | Standard CLI parsing |
| Tokenizer | tiktoken-rs | Token counting for 8K memory pruning |
| Image encoding | base64 | Image paste → base64 for multimodal messages |
| Web search | DuckDuckGo HTML scraping | Free, no API key, open-source approach |

## LLM Integration Details

Reference: https://platform.ai.tech.gov.sg/models/#models-api-reference

```
Base URL:  https://api.ai.tech.gov.sg/platform/models
Auth:      x-api-key: $ANTHROPIC_AUTH_TOKEN
Format:    OpenAI-compatible interface (POST /v1/chat/completions)
Models:    bedrock.claude-haiku-4-5 | bedrock.claude-sonnet-4-6 | bedrock.claude-opus-4-6
Streaming: SSE (stream: true)
```

### Request Format (OpenAI-compatible)

```json
POST {BASE_URL}/v1/chat/completions
Headers:
  x-api-key: <key>
  Content-Type: application/json

Body:
{
  "model": "bedrock.claude-sonnet-4-6",
  "messages": [
    {"role": "system", "content": "...core.md content..."},
    {"role": "user", "content": "..."}
  ],
  "stream": true,
  "max_tokens": 4096
}
```

### Multimodal (Image)

```json
{
  "role": "user",
  "content": [
    {"type": "text", "text": "describe this"},
    {"type": "image_url", "image_url": {"url": "data:image/png;base64,..."}}
  ]
}
```

## Dynamic Sub-Agent Architecture

```
User Input
    ↓
Orchestrator (Sonnet) — loads core.md + user message
    ↓
Planner: "decompose into N sub-tasks with role descriptions"
    ↓
┌──────────────────────────────────────────────┐
│  Dynamic Sub-Agent Pool (tokio::spawn each)  │
│                                              │
│  Agent 1: role="web researcher"    (Haiku)   │
│  Agent 2: role="code analyst"      (Sonnet)  │
│  Agent 3: role="summarizer"        (Haiku)   │
│  ...N agents, all receive core.md context    │
└──────────────────────────────────────────────┘
    ↓ (await all / timeout)
Synthesizer (Sonnet): merge sub-agent outputs → concise answer
    ↓
User Response
```

No agent types are hardcoded. The planner LLM call outputs a JSON array of `{role, task, model_hint}` and the pool spawns them dynamically.

## Notes

- **Platform AI gateway** — OpenAI-compatible interface, no direct AWS SDK needed. Auth via `x-api-key` header. Env vars already configured via `gt tools configure claude-code`. Ref: https://platform.ai.tech.gov.sg/models/#models-api-reference
- Sub-agents are ephemeral — spun up per-request, no persistent agent state. Only the orchestrator + memory persists.
- Multi-session: each session gets an ID, isolated memory + conversation history. Shared: config, skills, core.md.
- DuckDuckGo search via HTML scraping (`https://html.duckduckgo.com/html/?q=...`) — no API key, parse result snippets.
- Image paste: terminal supports base64 inline images (iTerm2/Kitty protocol detection). Fallback: file path reference.
- Alien pixel art: ratatui canvas widget renders ASCII/braille art at startup.

## Changelog

| Date | Change |
|------|--------|
| 2026-07-01 | Initial plan created from raw requirements |
| 2026-07-01 | Refined: dynamic sub-agents (no hardcoded types), core.md replaces CLAUDE.md, image paste in-scope, multi-session in-scope, alien pixel art TUI, resolved all OQs (Platform AI gateway, 8K memory, DuckDuckGo search) |
| 2026-07-01 | Added Platform AI API reference (OpenAI-compatible format, x-api-key auth, multimodal request format) |
