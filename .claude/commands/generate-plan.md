# Generate Plan

Create or refine a plan.md. Auto-detects mode: no plan.md exists → create from requirements; plan.md path provided or found → refine.

## Trigger

Keywords: "plan", "create plan", "refine plan"

## Modes

### Create Mode

1. Parse requirements from user input
2. Search codebase for relevant context
3. Generate folder structure and file list
4. Estimate story points: `(AC rows × 2) + Open Question rows`, rounded to nearest Fibonacci (1,2,3,5,8,13,21). 1 SP = 2 days.

### Refine Mode

1. Read existing plan.md
2. Apply user-requested changes
3. Update acceptance criteria
4. Cascade updates to related task files (reopen checkboxes with explanation if affected)

## Output

Save to `.docs/<folder-name>/plan.md` with sections:
- Summary
- Scope (in/out)
- Acceptance Criteria
- Open Questions
- Estimate
- Notes
- Changelog

## Readiness

Plan passes when: no blocking questions, clear summary, defined scope, concrete AC, no external blockers.
