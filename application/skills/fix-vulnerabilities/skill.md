---
name: fix-vulnerabilities
version: "1.0.0"
description: "Fetch and report GitLab security vulnerabilities (critical/high/medium/low) for repos"
triggers:
  - "vulnerability"
  - "vulnerabilities"
  - "security scan"
  - "CVE"
  - "security report"
  - "dependency scan"
parameters:
  - name: query
    type: string
    required: true
    description: "GitLab project path or MR URL to scan for vulnerabilities"
auth:
  - env: GITLAB_TOKEN
    description: "GitLab token with read_api scope"
commands:
  - name: preflight
    template: |
      echo "GITLAB_TOKEN: $([ -n "$GITLAB_TOKEN" ] && echo OK || echo MISSING)"
    timeout: 5
  - name: fetch-vulnerabilities
    template: |
      all_results="[]"
      for SEV in critical high medium low; do
        page=1
        while true; do
          chunk=$(curl -s -H "PRIVATE-TOKEN: $GITLAB_TOKEN" \
            "https://sgts.gitlab-dedicated.com/api/v4/projects/{project_path}/vulnerability_findings?severity[]=${SEV}&per_page=100&page=${page}")
          count=$(echo "$chunk" | jq 'length')
          [ "$count" -eq 0 ] && break
          all_results=$(echo "$all_results $chunk" | jq -s '.[0] + .[1]')
          page=$((page + 1))
          [ $page -gt 10 ] && break
        done
      done
      echo "$all_results" | jq '[.[] | select(.state == "detected") | {
        id, name, severity, state,
        scanner: .scanner.name,
        file: .location.file,
        dependency_pkg: .location.dependency.package.name,
        dependency_ver: .location.dependency.version,
        solution, identifiers: [.identifiers[].name]
      }] | group_by(.severity) | map({severity: .[0].severity, count: length, findings: .})'
    timeout: 60
---

# fix-vulnerabilities

Report GitLab security vulnerabilities. Report only — does NOT apply fixes.

## Workflow

1. Parse project path from URL, URL-encode it
2. Run preflight to verify token
3. Fetch all vulnerability findings across all severities
4. Report structured summary: CRITICAL/HIGH/MEDIUM/LOW counts + details
5. STOP: Do NOT apply fixes, run tests, or execute git commands
