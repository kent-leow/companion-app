#!/bin/bash
set -euo pipefail

TITLE=""
DESCRIPTION=""
ISSUE_TYPE="Story"
STORY_POINTS=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --title) TITLE="$2"; shift 2 ;;
    --description) DESCRIPTION="$2"; shift 2 ;;
    --issue-type) ISSUE_TYPE="$2"; shift 2 ;;
    --story-points) STORY_POINTS="$2"; shift 2 ;;
    *) shift ;;
  esac
done

if [ -z "$TITLE" ]; then
  echo "ERROR: --title required" >&2
  exit 1
fi

for var in JIRA_TOKEN JIRA_BASE_URL JIRA_PROJECT_KEY JIRA_EMAIL; do
  if [ -z "${!var:-}" ]; then
    echo "ERROR: $var not set" >&2
    exit 1
  fi
done

AUTH=$(echo -n "${JIRA_EMAIL}:${JIRA_TOKEN}" | base64)

curl -s -X POST "${JIRA_BASE_URL}/rest/api/3/issue" \
  -H "Authorization: Basic ${AUTH}" \
  -H "Content-Type: application/json" \
  -d "{
    \"fields\": {
      \"project\": {\"key\": \"${JIRA_PROJECT_KEY}\"},
      \"summary\": \"${TITLE}\",
      \"description\": {\"type\": \"doc\", \"version\": 1, \"content\": [{\"type\": \"paragraph\", \"content\": [{\"type\": \"text\", \"text\": \"${DESCRIPTION}\"}]}]},
      \"issuetype\": {\"name\": \"${ISSUE_TYPE}\"},
      \"customfield_10274\": ${STORY_POINTS}
    }
  }" | jq '{key: .key, id: .id, self: .self}'
