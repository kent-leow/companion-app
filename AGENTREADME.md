## Agent README

Monorepo with the Companion AI agent CLI.

### Structure

- `application/` — Node.js/TypeScript CLI agent. Has its own `package.json`, build tooling. Run all npm commands from this directory.
- `application/skills/` — Skill definitions (YAML frontmatter + prompt). No code changes needed to add skills.
- `.docs/` — Plans and task files.

### Quick Reference

| Action | Command | CWD |
|--------|---------|-----|
| Install deps | `npm install` | `application/` |
| Test | `npm test` | `application/` |
| Dev | `npm run dev` | `application/` |
| Build | `npm run build` | `application/` |
| Run built | `npm start` | `application/` |

### Key Points

- All app source lives in `application/src/`.
- Tests use `vitest` — run with `npm test` from `application/`.
- Entry point: `application/src/index.ts`.
- Orchestrator: `application/src/orchestrator/index.ts` — agentic loop.
- Skills: `application/skills/<name>/skill.md` — YAML frontmatter schema.
- LLM client: `application/src/llm/client.ts` — OpenAI-compatible, SSE streaming.
- Env vars required: `ANTHROPIC_BASE_URL`, `ANTHROPIC_AUTH_TOKEN`.
- Node.js 20+ required.
- Build produces single bundled JS at `application/dist/companion.js`.

### Architecture

```
User Input → TUI → Orchestrator → LLM (action prefixes) → Parse
                                    ↓
                    [TOOL:*]  → Skill Executor → feed result back → loop
                    [SUBAGENT:*] → Agent Pool (concurrent) → synthesize
                    [RESPOND] → stream to user → done
```
