# Plan: Add Skills to Companion App

## Summary

Transform 6 reference skill definitions (in `.docs/skills-setup/`) into runtime skills at `application/skills/<name>/skill.md`. The app's `SkillRegistry` auto-discovers folders with a `skill.md` — no Rust code changes needed.

## Scope

### In Scope

- Create `application/skills/<name>/skill.md` for each of the 6 skills:
  1. `figma-design-context` — Figma REST API design extraction
  2. `fix-vulnerabilities` — GitLab vulnerability fetching/reporting
  3. `git-apis` — Shared GitLab/GitHub REST operations
  4. `git-workflow` — Git branch/commit/push/MR/pipeline workflow
  5. `gitlab-mr-automation` — Self-contained GitLab MR lifecycle
  6. `jira-ticket` — Jira issue CRUD via REST API
- Each `skill.md` must follow the existing format: `# name`, description line, `## Commands` (with `{query}`/`{input}` placeholder), `## Prompt`
- Copy associated scripts from `.docs/skills-setup/<name>/scripts/` → `application/skills/<name>/scripts/`
- Copy associated references from `.docs/skills-setup/<name>/references/` → `application/skills/<name>/references/`

### Out of Scope

- Modifying Rust source code (`src/skills/`)
- Changing the planner prompt or routing logic
- Adding new env vars or credential management
- Writing tests for skills (skill loading already tested)
- Deploying or releasing

## Acceptance Criteria

- [ ] `application/skills/` contains 8 directories (2 existing + 6 new)
- [ ] Each new skill has `application/skills/<name>/skill.md` that parses correctly (valid `# name`, non-empty description, at least 1 command in `## Commands`, non-empty `## Prompt`)
- [ ] Skills with scripts have them at `application/skills/<name>/scripts/` (figma-design-context: 19 scripts, jira-ticket: 5 scripts)
- [ ] Skills with references have them at `application/skills/<name>/references/` (jira-ticket: 1 file)
- [ ] `cargo test` passes (skill loader loads all 8 skills without error)
- [ ] Each `skill.md` command uses `{query}` or `{input}` placeholder for argument substitution (per executor.rs template logic)

## Open Questions

All resolved:

1. ~~**Multi-command skills**~~ → **Resolved: Option C** — `## Commands` has a preflight command only; `## Prompt` embeds full workflow instructions. LLM picks which scripts to run.

2. ~~**Script paths**~~ → **Resolved: Relative paths** — All script references use `application/skills/<name>/scripts/<script>.sh` (repo-root-relative).

3. ~~**Skill interdependencies**~~ → **Resolved: Inline + standalone** — git-apis ops inlined into git-workflow and gitlab-mr-automation prompts. git-apis also kept as standalone skill for direct use.

## Estimate

| Factor | Count |
|---|---|
| AC rows | 6 |
| Open Questions | 3 |
| Raw score | (6 × 2) + 3 = 15 |
| **Fibonacci** | **13** |
| **Duration** | ~26 days |

> High estimate due to multi-command skill adaptation complexity and script path rewriting. If Open Questions are resolved (option C, relative paths, inline deps), reduces to **8 SP (~16 days)**.

## Notes

- The `SkillExecutor` (executor.rs) replaces `{query}` and `{input}` in command templates — skills with complex dispatch need a single entry-point command
- The `Planner` (planner.rs) already lists available skills by name and routes them to sub-agents — skills don't need to be simple single-shot commands
- Skills that are primarily "instruction sets" (like gitlab-mr-automation) are better suited as rich prompts with minimal commands (token check, discovery) rather than trying to encode the entire workflow as shell commands
- The 19 figma scripts and 5 jira scripts need to be executable (`chmod +x`)

## Changelog

| Date | Change |
|---|---|
| 2026-07-01 | Initial plan created from raw.md requirements |
| 2026-07-01 | Resolved all open questions; generated 7 task files (task-001 through task-007) |
