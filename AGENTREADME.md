## Agent README

Monorepo with two packages:

### Structure

- `application/` — Electron desktop app (the product). Has its own `package.json`, `tsconfig.json`, build tooling. Run all npm/test/build commands from this directory.
- `landing/` — Landing/download website.

### Quick Reference

| Action | Command | CWD |
|--------|---------|-----|
| Install | `npm install` | `application/` |
| Test | `npm test` | `application/` |
| Dev | `npm run dev` | `application/` |
| Build | `npx vite build` | `application/` |
| Type check | `npx tsc --noEmit` | `application/` |

### Key Points

- All app source lives in `application/src/`.
- Tests use Vitest.
- For detailed app architecture see `application/AGENTREADME.md`.
