# Fix Task

Address post-implementation issues with targeted fixes.

## Input

Issues from `issues.md` file or raw text input.

## Phases

### Phase 1 — Issue Ingestion
- Parse issues into numbered items
- Skim context from `plan.md` and task files

### Phase 2 — Folder & File Creation
- Generate `.docs/fix-{datetime}-{kebab-name}/fix-{datetime}.md`
- List numbered issue items

### Phase 3 — Fix Loop
For each issue:
1. Locate affected code
2. Apply targeted fix (minimal changes only)
3. Run tests
4. Mark item complete in fix file

### Phase 4 — Documentation Updates
- Update `task-NNN.md` and `plan.md` to reflect fixes
- Reopen tasks if needed
- Note architectural constraint gaps

### Phase 5 — Reporting
- Summary of all fixes applied
- Test results
- Remaining issues (if any)

## Constraints

- Minimal, targeted changes only
- No speculative reads or refactoring
- One issue at a time
