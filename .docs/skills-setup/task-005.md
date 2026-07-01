# Task 005 — Create git-workflow skill

## Goal
Add `git-workflow` skill with inlined git-apis operations for full branch/commit/push/MR/pipeline workflow.

## Prerequisites
- [ ] task-001 (git-apis skill exists as reference for inlining)

## Decisions
- Inline git-apis API call templates directly into git-workflow's prompt
- No external scripts — all commands are inline in the prompt
- Keep dependency note in prompt: "See also: git-apis skill for standalone API operations"

## Tasks
- [x] skill: Create `application/skills/git-workflow/skill.md`
  - `# git-workflow` heading
  - Description line: git branch setup, commit, push, MR creation, pipeline polling, review-thread lifecycle
  - `## Commands`: token preflight (`echo "GitLab: $([ -n "$GITLAB_TOKEN" ] && echo OK || echo MISSING)" && echo "GitHub: $([ -n "$GITHUB_TOKEN" ] && echo OK || echo MISSING)"` with `{query}` passthrough)
  - `## Prompt`: Full content from `.docs/skills-setup/git-workflow/SKILL.md` PLUS inlined git-apis operations (FETCH_DISCUSSIONS, POST_INLINE, POST_GENERAL, REPLY, RESOLVE, APPROVE curl templates) replacing the `CALL: git-apis` references
- [x] test: Verify skill loads — `cargo test` in `application/`

## Done When
- `application/skills/git-workflow/skill.md` exists and parses correctly
- No `CALL: git-apis` references remain — all API operations are inlined
- `cargo test` passes
