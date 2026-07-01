# Task 007 — Integration verification

## Goal
Verify all 8 skills (2 existing + 6 new) load correctly and the planner can route to them.

## Prerequisites
- [ ] task-001 (git-apis)
- [ ] task-002 (fix-vulnerabilities)
- [ ] task-003 (figma-design-context)
- [ ] task-004 (jira-ticket)
- [ ] task-005 (git-workflow)
- [ ] task-006 (gitlab-mr-automation)

## Tasks
- [x] verify: Run `cargo test` — all skill loader tests pass (3/3 pass)
- [x] verify: 8 skill directories confirmed (figma-design-context, fix-vulnerabilities, git-apis, git-workflow, gitlab-mr-automation, jira-ticket, web-fetch, web-search)
- [x] verify: Confirm each skill.md has valid structure:
  - `# name` matches directory name
  - Non-empty line after heading (description)
  - `## Commands` section with at least one non-empty line between ``` fences
  - `## Prompt` section with content
- [x] verify: Confirm script executability — `find application/skills -name "*.sh" ! -perm -u+x` returns empty
- [x] verify: No broken script path references — grep all skill.md for `.github/skills/` (should be 0 matches) — confirmed 0

## Done When
- `cargo test` passes
- All 8 skills discoverable at runtime
- No broken references to old `.github/skills/` paths
- All `.sh` files are executable
