# Task 001 — Create git-apis skill

## Goal
Add standalone `git-apis` skill that provides shared GitLab/GitHub REST API operations (fetch discussions, post comments, reply, resolve, approve).

## Prerequisites
- None

## Decisions
- Option C: `## Commands` has a preflight command only; `## Prompt` contains the full workflow instructions
- No scripts needed — commands are inline curl templates in the prompt

## Tasks
- [x] skill: Create `application/skills/git-apis/skill.md`
  - `# git-apis` heading
  - Description line: shared GitLab + GitHub REST API operations
  - `## Commands`: token preflight check (`echo "GitLab: $([ -n "$GITLAB_TOKEN" ] && echo OK || echo MISSING)" && echo "GitHub: $([ -n "$GITHUB_TOKEN" ] && echo OK || echo MISSING)"` with `{query}` placeholder for future use)
  - `## Prompt`: Full content from `.docs/skills-setup/git-apis/SKILL.md` (auth headers, FETCH_DISCUSSIONS, POST_INLINE, POST_GENERAL, REPLY, RESOLVE, APPROVE sections)
- [x] test: Verify skill loads — `cargo test` in `application/`

## Done When
- `application/skills/git-apis/skill.md` exists and parses (name, description, 1 command, non-empty prompt)
- `cargo test` passes
