# Generate Task

Break a plan.md into numbered task files, or refine an existing task.

## Trigger

Keywords: "generate tasks", "break down", "task from plan"

## Modes

### Generate Mode

Input: `plan.md` path
Output: Numbered `task-NNN.md` files

Requirements:
- Resolve all blocking questions in plan first
- Each task = independent vertical slice (half-day to two-day effort)
- Every logic file MUST have a test child

### Refine Mode

Input: `task-NNN.md` path + corrections
Output: Updated task file with changelog entry

## Task Template

```markdown
# Task NNN — <title>

## Goal
One-sentence deliverable statement.

## Prerequisites
- [ ] task-NNN (dependency description)

## Tasks
- [ ] <layer>: <action> — `path/to/file.ts`
- [ ] test: <test description> — `path/to/file.test.ts`

## Done When
- Observable completion criteria
```

## Rules

- Repo-root-relative file paths always
- No code invention
- Document changes in changelog
