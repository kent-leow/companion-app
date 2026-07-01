# Companion App

Terminal-based AI agent CLI powered by Claude (Platform AI gateway). Orchestrator pattern with dynamic sub-agents, structured action protocol, and self-evolving memory.

## Quick Start

```bash
cd application
npm install
npm run dev
```

Requires env vars (already configured via `gt tools configure claude-code`):
- `ANTHROPIC_BASE_URL` — Platform AI gateway endpoint
- `ANTHROPIC_AUTH_TOKEN` — API key

## Features

- Interactive TUI with alien pixel art splash and streaming markdown responses
- Claude Code-style agentic loop: parse → execute tools → loop until [RESPOND]
- Structured action protocol: `[TOOL]`, `[SUBAGENT]`, `[RESPOND]`, `[THINK]` prefixes
- Orchestrator decomposes complex queries into concurrent sub-agents
- Smart model routing: Haiku (simple) → Sonnet (standard) → Opus (complex)
- 11 skills loaded from `skills/<name>/skill.md` (5 zero-config + 6 auth-required)
- Context manager with rolling summary (sub-agents get focused context, not full history)
- Local memory (8K token budget, self-pruning)
- Image input support (multimodal via base64)
- File/folder context injection (`@src/index.ts`)
- Multi-session isolation

## Structure

```
application/    — Node.js/TypeScript CLI (the product)
.docs/          — Plans and task files
```

## Commands

| Action | Command | CWD |
|--------|---------|-----|
| Install | `npm install` | `application/` |
| Dev | `npm run dev` | `application/` |
| Test | `npm test` | `application/` |
| Build | `npm run build` | `application/` |
| Run built | `npm start` | `application/` |

## Adding Skills

Drop a folder in `application/skills/<name>/skill.md` with YAML frontmatter:

```yaml
---
name: my-skill
version: "1.0.0"
description: "What this skill does"
triggers:
  - "keyword1"
  - "keyword2"
parameters:
  - name: query
    type: string
    required: true
    description: "Input parameter"
commands:
  - name: run
    template: |
      curl -s "https://example.com/?q={query}"
    timeout: 15
---

# my-skill

Prompt instructions for the LLM when this skill is active.
```

No code changes needed — skills are discovered at startup via YAML frontmatter.
