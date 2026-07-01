# Task 006 — Create gitlab-mr-automation skill

## Goal
Add `gitlab-mr-automation` skill as self-contained GitLab MR lifecycle (branch → code → commit → push → MR → poll → fix → done).

## Prerequisites
- [ ] task-001 (git-apis skill exists as reference for inlining)

## Decisions
- Self-contained: inlines all needed git-apis operations directly
- No external scripts — all commands are inline in the prompt
- This skill is intentionally overlap-heavy with git-workflow — it's a single-command "do everything" skill vs git-workflow's composable steps

## Tasks
- [x] skill: Create `application/skills/gitlab-mr-automation/skill.md`
  - `# gitlab-mr-automation` heading
  - Description line: self-contained GitLab automation from ticket to merged MR
  - `## Commands`: token preflight (`echo "GITLAB_TOKEN: $([ -n "$GITLAB_TOKEN" ] && echo OK || echo MISSING)"` with `{query}` passthrough)
  - `## Prompt`: Full content from `.docs/skills-setup/gitlab-mr-automation/SKILL.md` (all 7 phases + constraints, already self-contained with inline API calls)
- [x] test: Verify skill loads — `cargo test` in `application/`

## Done When
- `application/skills/gitlab-mr-automation/skill.md` exists and parses correctly
- Skill is fully self-contained (no CALL references to other skills)
- `cargo test` passes
