---
name: jira-ticket
version: "1.0.0"
description: "Create/retrieve Jira issues: tickets, sub-tasks, story points, comments"
triggers:
  - "jira"
  - "ticket"
  - "story"
  - "sub-task"
  - "story points"
  - "atlassian"
  - "create ticket"
  - "issue"
parameters:
  - name: query
    type: string
    required: true
    description: "Jira request (e.g. 'create story for auth refactor, 5 SP')"
auth:
  - env: JIRA_TOKEN
    description: "Jira API token"
  - env: JIRA_BASE_URL
    description: "e.g. https://your-org.atlassian.net"
  - env: JIRA_PROJECT_KEY
    description: "e.g. PROJ"
  - env: JIRA_EMAIL
    description: "Atlassian account email"
commands:
  - name: preflight
    template: |
      echo "JIRA_TOKEN: $([ -n "$JIRA_TOKEN" ] && echo OK || echo MISSING)"
      echo "JIRA_BASE_URL: $([ -n "$JIRA_BASE_URL" ] && echo OK || echo MISSING)"
      echo "JIRA_PROJECT_KEY: $([ -n "$JIRA_PROJECT_KEY" ] && echo OK || echo MISSING)"
      echo "JIRA_EMAIL: $([ -n "$JIRA_EMAIL" ] && echo OK || echo MISSING)"
    timeout: 5
  - name: create-ticket
    template: |
      bash skills/jira-ticket/scripts/create-ticket.sh --title "{title}" --description "{description}" --issue-type "{issueType}" --story-points {storyPoints}
    timeout: 15
  - name: get-comments
    template: |
      bash skills/jira-ticket/scripts/get-comments.sh --issue-key "{issueKey}" --max-results 20
    timeout: 10
---

# jira-ticket

Jira ticket management specialist.

## Operations

| Operation | Script |
|-----------|--------|
| Create ticket | `create-ticket.sh --title --description --issue-type --story-points` |
| Get comments | `get-comments.sh --issue-key [--max-results N]` |
