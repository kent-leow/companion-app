# Execute Task

End-to-end task implementation: sync files, write code, run tests, mark checkboxes.

## Input

Task file path, e.g. `.docs/feature-name/task-002.md`

## Phases

### Phase 1 — Pre-flight
- Read the task and sibling files
- Check for open prerequisites
- Sync cross-references with warnings

### Phase 2 — Exploration
- Study all referenced files
- Find code analogues for conventions
- Identify reusable patterns

### Phase 3 — Implementation
- Write production code following project conventions
- Mark checkboxes immediately after each item
- Write tests matching existing patterns
- Run test suites

### Phase 4 — Verification
- Run full test suite
- Verify "Done When" conditions are met
- Check for stale cross-references

### Phase 5 — Completion
- Output summary: implemented files, test files, verification status, next steps

## Constraints

- Implement only what's listed in the task file
- Never mark tasks complete until code and tests pass
- Touch only affected lines in sibling files
- Search the codebase before asking for clarification
