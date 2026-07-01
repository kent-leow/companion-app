---
name: git-apis
version: "1.0.0"
description: "GitLab + GitHub REST API: fetch discussions, post comments, reply, resolve threads, approve MR/PR"
triggers:
  - "gitlab"
  - "github"
  - "merge request"
  - "pull request"
  - "MR"
  - "PR"
  - "review comments"
  - "approve"
  - "resolve thread"
  - "post comment"
  - "discussions"
parameters:
  - name: query
    type: string
    required: true
    description: "Full user request including URLs and context"
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

# git-apis

Git platform API specialist. Execute API operations via curl.

## Operations

| Operation | Description |
|-----------|-------------|
| FETCH_DISCUSSIONS | Get all MR/PR discussions (paginated) |
| POST_INLINE | Post inline comment on a diff line |
| POST_GENERAL | Post general comment |
| REPLY | Reply to an existing thread |
| RESOLVE | Mark thread as resolved |
| APPROVE | Approve the MR/PR |

## Auth Headers

| Platform | Header |
|---|---|
| GitLab | `PRIVATE-TOKEN: $GITLAB_TOKEN` |
| GitHub | `Authorization: Bearer $GITHUB_TOKEN` |

Use [TOOL:run-command] to execute curl commands for each operation.
