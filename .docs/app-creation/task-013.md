# Task 013 — Auth-Required Skills

## Goal
Create the 6 auth-required skill.md files with their supporting scripts: figma-design-context, fix-vulnerabilities, git-apis, git-workflow, gitlab-mr-automation, jira-ticket.

## Prerequisites
- [ ] task-004 (skill schema/loader)
- [ ] task-005 (skill executor)
- [ ] task-012 (default skills pattern established)

## Tasks
- [ ] skill: Create figma-design-context/skill.md + scripts (get-metadata.sh, get-screenshot.sh, get-design-context.sh) — `application/skills/figma-design-context/`
- [ ] skill: Create fix-vulnerabilities/skill.md (GitLab vulnerability fetch, report-only) — `application/skills/fix-vulnerabilities/skill.md`
- [ ] skill: Create git-apis/skill.md (discussions, comments, resolve, approve) — `application/skills/git-apis/skill.md`
- [ ] skill: Create git-workflow/skill.md (branch, commit, push, MR, pipeline poll) — `application/skills/git-workflow/skill.md`
- [ ] skill: Create gitlab-mr-automation/skill.md (full lifecycle) — `application/skills/gitlab-mr-automation/skill.md`
- [ ] skill: Create jira-ticket/skill.md + scripts (create-ticket.sh, get-comments.sh, get-fields.sh) — `application/skills/jira-ticket/`
- [ ] test: All 6 auth skills load via registry (with auth warnings for missing tokens) — `application/tests/auth-skills.test.ts`
- [ ] test: Preflight commands detect missing env vars correctly — `application/tests/auth-skills.test.ts`

## Done When
- All 6 skills have valid YAML frontmatter matching SkillDef schema
- Registry logs warnings for skills with missing auth env vars
- Preflight commands report OK/MISSING status correctly
- Shell scripts in figma + jira directories are executable
- All tests pass
