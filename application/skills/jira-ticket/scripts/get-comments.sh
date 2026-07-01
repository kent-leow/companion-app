#!/bin/bash
set -euo pipefail

ISSUE_KEY=""
MAX_RESULTS=20

while [[ $# -gt 0 ]]; do
  case "$1" in
    --issue-key) ISSUE_KEY="$2"; shift 2 ;;
    --max-results) MAX_RESULTS="$2"; shift 2 ;;
    *) shift ;;
  esac
done

if [ -z "$ISSUE_KEY" ]; then
  echo "ERROR: --issue-key required" >&2
  exit 1
fi

for var in JIRA_TOKEN JIRA_BASE_URL JIRA_EMAIL; do
  if [ -z "${!var:-}" ]; then
    echo "ERROR: $var not set" >&2
    exit 1
  fi
done

AUTH=$(echo -n "${JIRA_EMAIL}:${JIRA_TOKEN}" | base64)

curl -s "${JIRA_BASE_URL}/rest/api/3/issue/${ISSUE_KEY}/comment?maxResults=${MAX_RESULTS}" \
  -H "Authorization: Basic ${AUTH}" \
  -H "Content-Type: application/json" \
  | jq '[.comments[] | {id: .id, author: .author.displayName, created: .created, body: .body.content[0].content[0].text}]'
