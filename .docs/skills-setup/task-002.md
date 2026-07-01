# Task 002 — Create fix-vulnerabilities skill

## Goal
Add `fix-vulnerabilities` skill that fetches and reports GitLab security vulnerabilities for GOBIZ repos.

## Prerequisites
- None

## Decisions
- Option C: `## Commands` has token preflight only; `## Prompt` has full workflow
- No external scripts — all commands are inline curl/jq in the prompt

## Tasks
- [x] skill: Create `application/skills/fix-vulnerabilities/skill.md`
  - `# fix-vulnerabilities` heading
  - Description line: fetch and report GitLab security vulnerabilities
  - `## Commands`: token check (`echo "GITLAB_TOKEN: $([ -n "$GITLAB_TOKEN" ] && echo OK || echo MISSING)"` with `{query}` passthrough)
  - `## Prompt`: Full content from `.docs/skills-setup/fix-vulnerabilities/SKILL.md` (sandbox note, prerequisite, repo map, steps, errors)
- [x] test: Verify skill loads — `cargo test` in `application/`

## Done When
- `application/skills/fix-vulnerabilities/skill.md` exists and parses correctly
- `cargo test` passes
