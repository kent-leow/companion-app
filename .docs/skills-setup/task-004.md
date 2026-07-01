# Task 004 — Create jira-ticket skill

## Goal
Add `jira-ticket` skill with 5 shell scripts and 1 reference file for Jira issue CRUD.

## Prerequisites
- None

## Decisions
- Option C: `## Commands` has token preflight; `## Prompt` has full workflow referencing scripts
- Script paths updated to `application/skills/jira-ticket/scripts/<name>.sh`

## Tasks
- [x] skill: Create `application/skills/jira-ticket/skill.md`
  - `# jira-ticket` heading
  - Description line: create/retrieve Jira issues via REST API
  - `## Commands`: env var check (`echo "JIRA_TOKEN: $([ -n "$JIRA_TOKEN" ] && echo OK || echo MISSING)" && echo "JIRA_BASE_URL: $([ -n "$JIRA_BASE_URL" ] && echo OK || echo MISSING)"` with `{query}` passthrough)
  - `## Prompt`: Full content from `.docs/skills-setup/jira-ticket/SKILL.md` with script paths rewritten from `.github/skills/jira-ticket/scripts/` → `application/skills/jira-ticket/scripts/`
- [x] scripts: Copy all 5 scripts from `.docs/skills-setup/jira-ticket/scripts/` → `application/skills/jira-ticket/scripts/`
- [x] scripts: Ensure all scripts are executable (`chmod +x`)
- [x] references: Copy `.docs/skills-setup/jira-ticket/references/` → `application/skills/jira-ticket/references/`
- [x] test: Verify skill loads — `cargo test` in `application/`

## Done When
- `application/skills/jira-ticket/skill.md` exists and parses correctly
- `application/skills/jira-ticket/scripts/` contains 5 executable scripts
- `application/skills/jira-ticket/references/jira-api.md` exists
- `cargo test` passes
