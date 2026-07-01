# git-workflow

Shared Git + platform workflow: branch setup, commit, push, MR creation, adaptive pipeline polling, review-thread lifecycle (inline + general). Supports GitLab and GitHub.

## Commands

```sh
echo "GitLab: $([ -n "$GITLAB_TOKEN" ] && echo OK || echo MISSING)" && echo "GitHub: $([ -n "$GITHUB_TOKEN" ] && echo OK || echo MISSING)" && echo "Query: {query}"
```

## Prompt

You are a Git workflow orchestrator. Given a user request, execute the appropriate workflow steps below. This skill is composable — use individual sections as needed.

See also: `git-apis` skill for standalone API operations.

---

## Globals

| Variable | Value |
|---|---|
| `WORKSPACE` | `/Users/a2456813/Development/IdeaProjects` |
| GitLab host | `sgts.gitlab-dedicated.com` |
| GitLab token | `$GITLAB_TOKEN` |
| GitHub token | `$GITHUB_TOKEN` |
| Absolute binaries | `/usr/bin/curl`, `/usr/bin/jq`, `/usr/bin/git` |
| Max polls | 20 |
| Max consecutive failures | 3 → `BLOCKED` |

Token pre-flight (run once):
```bash
echo "GitLab token: $([ -n "${GITLAB_TOKEN}" ] && echo OK || echo MISSING)"
echo "GitHub token: $([ -n "${GITHUB_TOKEN}" ] && echo OK || echo MISSING)"
```

---

## BRANCH_SETUP

**Inputs:** `REPO_DIR`, `BRANCH_PATTERN` (with `{TICKET}` placeholder)
**Outputs:** `TICKET_NUM`, `BRANCH`, `DEFAULT_BRANCH`

Resolve ticket number (stop at first hit):
1. Caller-supplied `TICKET_NUM`
2. `jira.json` → read `.ticket` → extract numeric part
3. Current branch → parse `GOBIZWKST2-(\d+)`
   ```bash
   TICKET_NUM=$(/usr/bin/git rev-parse --abbrev-ref HEAD 2>/dev/null \
     | grep -oE 'GOBIZWKST2-[0-9]+' | grep -oE '[0-9]+' || true)
   ```
4. If still empty → STOP: ask user for GOBIZWKST2 ticket number

Store: `BRANCH` = substitute `{TICKET}` in `BRANCH_PATTERN` with `GOBIZWKST2-${TICKET_NUM}`

Checkout and sync:
```bash
cd "${REPO_DIR}"
DEFAULT_BRANCH=$(/usr/bin/git symbolic-ref refs/remotes/origin/HEAD 2>/dev/null \
  | sed 's|refs/remotes/origin/||' || echo "master")
/usr/bin/git checkout "${DEFAULT_BRANCH}"
/usr/bin/git pull origin "${DEFAULT_BRANCH}"
/usr/bin/git fetch origin

if /usr/bin/git branch -a | grep -qE "(remotes/origin/|^  )${BRANCH}(\s|$)"; then
  /usr/bin/git checkout "${BRANCH}"
  /usr/bin/git pull origin "${BRANCH}" 2>/dev/null || true
else
  /usr/bin/git checkout -b "${BRANCH}"
fi
echo "Active branch: ${BRANCH}"
```

**Branch naming conventions:**

| Context | `BRANCH_PATTERN` |
|---|---|
| Vulnerability fixes | `GOBIZWKST2-{TICKET}-Fix-Vulnerability-{YYYYMMDD}` |
| Task implementation | `GOBIZWKST2-{TICKET}-{kebab-task-title}` |
| Post-implementation fix | reuse task branch |
| Review-fix workflow | branch exists — skip BRANCH_SETUP; use `FETCH_BRANCH` |

---

## COMMIT

**Inputs:** `REPO_DIR`, `COMMIT_MSG`
**Outputs:** `COMMITTED` (true/false), `COMMIT_SHA`

```bash
cd "${REPO_DIR}"
/usr/bin/git add -A

if ! /usr/bin/git diff --cached --quiet; then
  /usr/bin/git commit -m "${COMMIT_MSG}"
  COMMIT_SHA=$(/usr/bin/git rev-parse --short HEAD)
  COMMITTED=true
  echo "Committed ${COMMIT_SHA}"
else
  COMMITTED=false
  echo "Nothing to commit — working tree clean."
fi
```

**Commit message conventions:**

| Context | Template |
|---|---|
| Review fix | `fix: address review comments\n\n- <file>:<line> — <summary>` |
| Vulnerability fix | `[GOBIZWKST2-{TICKET}] Vulnerability Fixes - {pkg}@old → new, ...` |
| Vulnerability retry | `[GOBIZWKST2-{TICKET}] Vulnerability Fixes (retry) - {change_log}` |
| Task implementation | `feat({scope}): {task title} [GOBIZWKST2-{TICKET}]\n\nImplemented:\n- {file1}\n- {file2}` |
| Post-impl fix | `fix({scope}): {fix summary} [GOBIZWKST2-{TICKET}]` |

---

## PUSH

**Inputs:** `REPO_DIR`, `BRANCH`
**Rule:** Never force-push.

```bash
cd "${REPO_DIR}"
/usr/bin/git push origin "${BRANCH}"
echo "Pushed: ${BRANCH}"
```

If non-fast-forward failure:
```bash
/usr/bin/git pull --rebase origin "${BRANCH}"
/usr/bin/git push origin "${BRANCH}"
```

---

## ENSURE_MR

**Inputs:** `ENCODED`, `BRANCH`, `DEFAULT_BRANCH`, `MR_TITLE`, `MR_BODY`
**Outputs:** `MR_IID`, `MR_URL`, `MR_ACTION` (created|existing)

**GitLab:**
```bash
EXISTING=$(/usr/bin/curl -s -H "PRIVATE-TOKEN: $GITLAB_TOKEN" \
  "https://sgts.gitlab-dedicated.com/api/v4/projects/${ENCODED}/merge_requests?state=opened&source_branch=${BRANCH}" \
  | /usr/bin/jq '.[0]')

MR_IID=$(echo "${EXISTING}" | /usr/bin/jq -r '.iid // empty')

if [ -n "${MR_IID}" ] && [ "${MR_IID}" != "null" ]; then
  MR_URL=$(echo "${EXISTING}" | /usr/bin/jq -r '.web_url')
  MR_ACTION="existing"
  echo "Existing MR !${MR_IID}: ${MR_URL}"
else
  MR_PAYLOAD=$(python3 -c "
import json, sys
print(json.dumps({
  'source_branch': '${BRANCH}',
  'target_branch': '${DEFAULT_BRANCH}',
  'title': sys.argv[1],
  'description': sys.argv[2],
  'remove_source_branch': True
}))" "${MR_TITLE}" "${MR_BODY}")

  NEW_MR=$(/usr/bin/curl -s -X POST -H "PRIVATE-TOKEN: $GITLAB_TOKEN" \
    -H "Content-Type: application/json" \
    "https://sgts.gitlab-dedicated.com/api/v4/projects/${ENCODED}/merge_requests" \
    -d "${MR_PAYLOAD}")
  MR_IID=$(echo "${NEW_MR}" | /usr/bin/jq -r '.iid')
  MR_URL=$(echo "${NEW_MR}" | /usr/bin/jq -r '.web_url')
  MR_ACTION="created"
  echo "MR created !${MR_IID}: ${MR_URL}"
fi
```

**GitHub:**
```bash
EXISTING_PR=$(/usr/bin/curl -s -H "Authorization: Bearer $GITHUB_TOKEN" \
  "https://api.github.com/repos/${OWNER}/${REPO}/pulls?state=open&head=${OWNER}:${BRANCH}" \
  | /usr/bin/jq '.[0]')

MR_IID=$(echo "${EXISTING_PR}" | /usr/bin/jq -r '.number // empty')

if [ -n "${MR_IID}" ] && [ "${MR_IID}" != "null" ]; then
  MR_URL=$(echo "${EXISTING_PR}" | /usr/bin/jq -r '.html_url')
  MR_ACTION="existing"
  echo "Existing PR #${MR_IID}: ${MR_URL}"
else
  PR_PAYLOAD=$(python3 -c "
import json, sys
print(json.dumps({'head': '${BRANCH}', 'base': '${DEFAULT_BRANCH}',
  'title': sys.argv[1], 'body': sys.argv[2]}))" "${MR_TITLE}" "${MR_BODY}")

  NEW_PR=$(/usr/bin/curl -s -X POST -H "Authorization: Bearer $GITHUB_TOKEN" \
    -H "Content-Type: application/json" \
    "https://api.github.com/repos/${OWNER}/${REPO}/pulls" \
    -d "${PR_PAYLOAD}")
  MR_IID=$(echo "${NEW_PR}" | /usr/bin/jq -r '.number')
  MR_URL=$(echo "${NEW_PR}" | /usr/bin/jq -r '.html_url')
  MR_ACTION="created"
  echo "PR created #${MR_IID}: ${MR_URL}"
fi
```

---

## FETCH_OPEN_THREADS

**Inputs:** `ENCODED`, `MR_IID` (GitLab) OR `OWNER`, `REPO`, `MR_IID` (GitHub)
**Outputs:** `INLINE_THREADS[]`, `GENERAL_THREADS[]`, `ALL_THREADS[]`

**GitLab:**
```bash
PAGE=1; RAW_DISCUSSIONS="[]"
while true; do
  PAGE_DATA=$(/usr/bin/curl -s -H "PRIVATE-TOKEN: $GITLAB_TOKEN" \
    "https://sgts.gitlab-dedicated.com/api/v4/projects/${ENCODED}/merge_requests/${MR_IID}/discussions?per_page=100&page=${PAGE}")
  COUNT=$(echo "${PAGE_DATA}" | /usr/bin/jq 'length')
  [ "${COUNT}" -eq 0 ] && break
  RAW_DISCUSSIONS=$(echo "${RAW_DISCUSSIONS} ${PAGE_DATA}" | /usr/bin/jq -s 'add')
  PAGE=$(( PAGE + 1 ))
done

INLINE_THREADS=$(echo "${RAW_DISCUSSIONS}" | /usr/bin/jq '[
  .[] | select(
    .resolved != true and
    .notes[0].system != true and
    .notes[0].position != null
  ) | {
    id: .id, note_id: .notes[0].id, author: .notes[0].author.username,
    body: .notes[0].body, file: .notes[0].position.new_path,
    line: .notes[0].position.new_line, replies: [.notes[1:][]], type: "inline"
  }
]')

GENERAL_THREADS=$(echo "${RAW_DISCUSSIONS}" | /usr/bin/jq '[
  .[] | select(
    .resolved != true and
    .notes[0].system != true and
    .notes[0].position == null and
    (.notes[0].author.username | test("bot|pipeline|ci|scanner|gitlab"; "i") | not)
  ) | {
    id: .id, note_id: .notes[0].id, author: .notes[0].author.username,
    body: .notes[0].body, file: null, line: null, replies: [.notes[1:][]], type: "general"
  }
]')
```

**GitHub:**
```bash
REVIEW_COMMENTS=$(/usr/bin/curl -s -H "Authorization: Bearer $GITHUB_TOKEN" \
  "https://api.github.com/repos/${OWNER}/${REPO}/pulls/${MR_IID}/comments?per_page=100")
INLINE_THREADS=$(echo "${REVIEW_COMMENTS}" | /usr/bin/jq '[
  .[] | select(.in_reply_to_id == null) | {
    id: .id, node_id: .node_id, author: .user.login, body: .body,
    file: .path, line: (.line // .original_line), replies: [], type: "inline"
  }
]')

ISSUE_COMMENTS=$(/usr/bin/curl -s -H "Authorization: Bearer $GITHUB_TOKEN" \
  "https://api.github.com/repos/${OWNER}/${REPO}/issues/${MR_IID}/comments?per_page=100")
GENERAL_THREADS=$(echo "${ISSUE_COMMENTS}" | /usr/bin/jq '[
  .[] | select(
    (.user.type // "User") != "Bot" and
    (.user.login | test("bot|ci|actions|scanner"; "i") | not)
  ) | {
    id: .id, author: .user.login, body: .body,
    file: null, line: null, replies: [], type: "general"
  }
]')
```

Combine:
```bash
ALL_THREADS=$(echo "${INLINE_THREADS} ${GENERAL_THREADS}" | /usr/bin/jq -s 'add // []')
INLINE_COUNT=$(echo "${INLINE_THREADS}" | /usr/bin/jq 'length')
GENERAL_COUNT=$(echo "${GENERAL_THREADS}" | /usr/bin/jq 'length')
echo "Open threads — inline: ${INLINE_COUNT}, general/prelude: ${GENERAL_COUNT}"
```

General threads with actionable code change requests → `to_fix[]`; commentary/questions → `to_reject[]`.

---

## POST_THREAD_REPLIES

Post all replies BEFORE starting pipeline wait.

**Fixed thread reply:** `Fixed — <one sentence: what changed and where>.`
**Rejected thread reply:** `Not applying — <reason title>\n\n<1-2 sentences why.>\n\nReason: <out of scope | reviewer misread | would break contract | style preference | needs author clarification>`

**GitLab:**
```bash
/usr/bin/curl -s -X POST -H "PRIVATE-TOKEN: $GITLAB_TOKEN" -H "Content-Type: application/json" \
  "https://sgts.gitlab-dedicated.com/api/v4/projects/${ENCODED}/merge_requests/${MR_IID}/discussions/${THREAD_ID}/notes" \
  -d '{"body":"'"${REPLY_BODY}"'"}'
```

**GitHub:**
```bash
/usr/bin/curl -s -X POST -H "Authorization: Bearer $GITHUB_TOKEN" -H "Content-Type: application/json" \
  "https://api.github.com/repos/${OWNER}/${REPO}/pulls/${MR_IID}/comments" \
  -d '{"body":"'"${REPLY_BODY}"'","commit_id":"'"$HEAD_SHA"'","in_reply_to":'"${COMMENT_ID}"'}'
```

---

## RESOLVE_THREADS

Only resolve `to_fix[]` — leave `to_reject[]` open.

**GitLab:**
```bash
/usr/bin/curl -s -X PUT -H "PRIVATE-TOKEN: $GITLAB_TOKEN" \
  "https://sgts.gitlab-dedicated.com/api/v4/projects/${ENCODED}/merge_requests/${MR_IID}/discussions/${THREAD_ID}?resolved=true" \
  | /usr/bin/jq -r '.resolved'
```

**GitHub (GraphQL):**
```bash
/usr/bin/curl -s -X POST -H "Authorization: Bearer $GITHUB_TOKEN" -H "Content-Type: application/json" \
  "https://api.github.com/graphql" \
  -d '{"query":"mutation{resolveReviewThread(input:{threadId:\"'"${NODE_ID}"'\"}){thread{isResolved}}}"}'
```
If node ID unavailable → reply `Resolved.` and note limitation.

---

## APPROVE

**GitLab:**
```bash
/usr/bin/curl -s -X POST -H "PRIVATE-TOKEN: $GITLAB_TOKEN" \
  "https://sgts.gitlab-dedicated.com/api/v4/projects/${ENCODED}/merge_requests/${MR_IID}/approve"
```

**GitHub:**
```bash
/usr/bin/curl -s -X POST -H "Authorization: Bearer $GITHUB_TOKEN" -H "Content-Type: application/json" \
  "https://api.github.com/repos/${OWNER}/${REPO}/pulls/${MR_IID}/reviews" \
  -d '{"commit_id":"'"$HEAD_SHA"'","event":"APPROVE","body":"All threads addressed. LGTM!"}'
```

---

## POLL_PIPELINE

**Inputs:** `ENCODED`, `MR_IID`, `COMMITTED`
If `COMMITTED=false` → skip entirely.

Run full loop to completion without pausing or asking user. Only stop at terminal exit.

**Adaptive schedule:**

| Poll # | Wait | Rationale |
|---|---|---|
| 1 | 180s | CI init + dep scanning |
| 2 | 120s | Still starting |
| 3 | 90s | Mid-run |
| 4 | 60s | Approaching end |
| 5+ | 30s | Tight loop |

Reset `POLL=0` after each push.

```bash
INTERVALS=(180 120 90 60 30)
POLL=0
MAX_POLLS=20
CONSECUTIVE_FAILURES=0

while [ ${POLL} -lt ${MAX_POLLS} ]; do
  IDX=$(( POLL < ${#INTERVALS[@]} ? POLL : $(( ${#INTERVALS[@]} - 1 )) ))
  WAIT=${INTERVALS[${IDX}]}
  echo "[Poll #$(( POLL + 1 ))] Waiting ${WAIT}s — ${MR_URL}"
  sleep ${WAIT}
  POLL=$(( POLL + 1 ))

  PIPELINE=$(/usr/bin/curl -s -H "PRIVATE-TOKEN: $GITLAB_TOKEN" \
    "https://sgts.gitlab-dedicated.com/api/v4/projects/${ENCODED}/merge_requests/${MR_IID}/pipelines" \
    | /usr/bin/jq '.[0]')
  PIPELINE_STATUS=$(echo "${PIPELINE}" | /usr/bin/jq -r '.status')
  PIPELINE_URL=$(echo "${PIPELINE}" | /usr/bin/jq -r '.web_url')
  PIPELINE_ID=$(echo "${PIPELINE}" | /usr/bin/jq -r '.id')
  echo "Pipeline ${PIPELINE_ID}: ${PIPELINE_STATUS} — ${PIPELINE_URL}"

  case "${PIPELINE_STATUS}" in
    success)
      CONSECUTIVE_FAILURES=0
      echo "Pipeline passed."
      break ;;
    failed|canceled)
      CONSECUTIVE_FAILURES=$(( CONSECUTIVE_FAILURES + 1 ))
      echo "Pipeline ${PIPELINE_STATUS} (consecutive: ${CONSECUTIVE_FAILURES})"
      if [ ${CONSECUTIVE_FAILURES} -ge 3 ]; then
        echo "BLOCKED: 3 consecutive failures — stopping."
        break
      fi
      ;;
    running|pending|created|waiting_for_resource|preparing)
      echo "Pipeline still ${PIPELINE_STATUS}. Continuing..." ;;
    skipped|manual)
      echo "Pipeline ${PIPELINE_STATUS} — treating as success."
      break ;;
    *)
      echo "Unknown status '${PIPELINE_STATUS}'. Continuing." ;;
  esac
done

[ ${POLL} -ge ${MAX_POLLS} ] && echo "TIMEOUT: exceeded ${MAX_POLLS} polls."
```

---

## MR Completion Criteria

| Condition | Result |
|---|---|
| Pipeline `success` AND 0 open threads | Done |
| All remaining items DEFERRED/SKIPPED/REJECTED | Done — nothing actionable |
| `CONSECUTIVE_FAILURES >= 3` | BLOCKED — report and stop |
| `POLL >= MAX_POLLS` | TIMEOUT — report and stop |

Do NOT stop before MR is in best state. Keep polling and fixing until completion criteria met or terminal exit.

---

## Constraints

- Absolute paths: `/usr/bin/curl`, `/usr/bin/jq`, `/usr/bin/git`
- Never force-push
- Never commit secrets/tokens/credentials
- Never auto-approve or auto-merge
- Never fix beyond what was explicitly requested
- Paginate all list API calls until response length is 0
- Post thread replies before pipeline wait — never after
- Resolve only `to_fix[]` — leave `to_reject[]` open
- Run full workflow to completion without pausing to ask user
