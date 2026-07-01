---
name: git-workflow
version: "1.0.0"
description: "Git workflow automation: branch setup, commit, push, MR creation, pipeline polling"
triggers:
  - "branch"
  - "commit"
  - "push"
  - "create MR"
  - "create PR"
  - "pipeline"
  - "CI"
  - "review fix"
parameters:
  - name: query
    type: string
    required: true
    description: "Workflow request (e.g. 'create branch and MR for ticket PROJ-123')"
auth:
  - env: GITLAB_TOKEN
    description: "GitLab personal access token"
  - env: GITHUB_TOKEN
    description: "GitHub personal access token"
commands:
  - name: preflight
    template: |
      echo "GitLab: $([ -n "$GITLAB_TOKEN" ] && echo OK || echo MISSING)"
      echo "GitHub: $([ -n "$GITHUB_TOKEN" ] && echo OK || echo MISSING)"
    timeout: 5
---

# git-workflow

Git workflow orchestrator.

## Phases

| Phase | Description |
|-------|-------------|
| BRANCH_SETUP | Create/checkout branch |
| COMMIT | Stage + commit with conventional message |
| PUSH | Push to remote (never force-push) |
| ENSURE_MR | Create or find existing MR/PR |
| POLL_PIPELINE | Adaptive polling until success/failure/timeout |

## Constraints

- Never force-push
- Never commit secrets/tokens
- Never auto-approve or auto-merge
