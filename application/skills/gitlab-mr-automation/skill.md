---
name: gitlab-mr-automation
version: "1.0.0"
description: "Self-contained GitLab MR automation: branch, commit, push, MR, poll pipeline, resolve threads"
triggers:
  - "automate MR"
  - "submit code"
  - "implement task"
  - "fix review"
  - "resolve threads"
  - "pipeline fix"
  - "MR lifecycle"
parameters:
  - name: query
    type: string
    required: true
    description: "Full request with repo dir, branch pattern, commit msg, MR title"
auth:
  - env: GITLAB_TOKEN
    description: "GitLab personal access token"
commands:
  - name: preflight
    template: |
      echo "GITLAB_TOKEN: $([ -n "$GITLAB_TOKEN" ] && echo OK || echo MISSING)"
    timeout: 5
---

# gitlab-mr-automation

Full GitLab MR lifecycle: branch → commit → push → MR → poll → resolve.

## Terminal States

| Condition | Status |
|---|---|
| Pipeline success + 0 open threads | SUCCESS |
| 3 consecutive pipeline failures | BLOCKED |
| 20 polls exceeded | TIMEOUT |

## Constraints

- Never force-push
- Never commit secrets
- Never auto-approve or auto-merge
- Run full loop to completion
