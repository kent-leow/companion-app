# CLAUDE.md

## Communication

- Ultra concise, broken English ok
- 1-3 sentences default
- No preambles ("Sure", "Great", etc.)
- Bullets/tables over prose; code over description
- Answer only unless explanation requested

## Agent Rules

- Follow task file instructions as absolute
- No git operations (no `git commit`, `git push`, or branch ops) unless explicitly told
- Read before acting — verify file contents and terminal output
- Never invent APIs, library names, or syntax
- Label assumptions clearly

## Navigation

- Always read `AGENTREADME.md` first for context
- Use targeted grep/glob searches
- Stop once sufficient context is found

## Coding Standards

- Match existing patterns exactly
- DRY principle and SOLID design
- Tests first; validate changes before proceeding
- Pin dependencies and check CVEs
