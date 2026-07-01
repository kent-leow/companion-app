# git-apis

Shared GitLab + GitHub REST API operations: fetch discussions, post inline/general comments, reply, resolve threads, approve MR/PR.

## Commands

```sh
echo "GitLab: $([ -n "$GITLAB_TOKEN" ] && echo OK || echo MISSING)" && echo "GitHub: $([ -n "$GITHUB_TOKEN" ] && echo OK || echo MISSING)" && echo "Query: {query}"
```

## Prompt

You are a Git platform API specialist. Given a user request, execute the appropriate API operation below.

## Auth Headers

| Platform | Header |
|---|---|
| GitLab | `PRIVATE-TOKEN: $GITLAB_TOKEN` |
| GitHub | `Authorization: Bearer $GITHUB_TOKEN` |

---

## FETCH_DISCUSSIONS

Fetch all MR/PR discussions.

**GitLab:**
```bash
DISCUSSIONS=$(curl -s -H "PRIVATE-TOKEN: $GITLAB_TOKEN" \
  "https://sgts.gitlab-dedicated.com/api/v4/projects/<encoded-project>/merge_requests/<MR_ID>/discussions?per_page=100")
```

**GitHub:**
```bash
REVIEW_COMMENTS=$(curl -s -H "Authorization: Bearer $GITHUB_TOKEN" \
  "https://api.github.com/repos/<owner>/<repo>/pulls/<PR_ID>/comments?per_page=100")
ISSUE_COMMENTS=$(curl -s -H "Authorization: Bearer $GITHUB_TOKEN" \
  "https://api.github.com/repos/<owner>/<repo>/issues/<PR_ID>/comments?per_page=100")
```

---

## POST_INLINE

Post inline comment on a diff line.

**GitLab:**
```bash
curl -s -X POST -H "PRIVATE-TOKEN: $GITLAB_TOKEN" -H "Content-Type: application/json" \
  "https://sgts.gitlab-dedicated.com/api/v4/projects/<encoded-project>/merge_requests/<MR_ID>/discussions" \
  -d '{"body":"<comment>","position":{"position_type":"text","base_sha":"'"$BASE_SHA"'","start_sha":"'"$BASE_SHA"'","head_sha":"'"$HEAD_SHA"'","old_path":"<file>","new_path":"<file>","new_line":<N>}}'
```
Line range: replace `"new_line":<N>` with `"line_range":{"start":{"type":"new","new_line":<S>},"end":{"type":"new","new_line":<E>}}`

**GitHub:**
```bash
curl -s -X POST -H "Authorization: Bearer $GITHUB_TOKEN" -H "Content-Type: application/json" \
  "https://api.github.com/repos/<owner>/<repo>/pulls/<PR_ID>/comments" \
  -d '{"body":"<comment>","commit_id":"'"$HEAD_SHA"'","path":"<file>","line":<N>,"side":"RIGHT"}'
```
Line range: add `"start_line":<S>,"start_side":"RIGHT"`

---

## POST_GENERAL

Post general (non-inline) comment.

**GitLab:**
```bash
curl -s -X POST -H "PRIVATE-TOKEN: $GITLAB_TOKEN" -H "Content-Type: application/json" \
  "https://sgts.gitlab-dedicated.com/api/v4/projects/<encoded-project>/merge_requests/<MR_ID>/notes" \
  -d '{"body":"<comment>"}'
```

**GitHub:**
```bash
curl -s -X POST -H "Authorization: Bearer $GITHUB_TOKEN" -H "Content-Type: application/json" \
  "https://api.github.com/repos/<owner>/<repo>/issues/<PR_ID>/comments" \
  -d '{"body":"<comment>"}'
```

---

## REPLY

Reply to an existing thread.

**GitLab:**
```bash
curl -s -X POST -H "PRIVATE-TOKEN: $GITLAB_TOKEN" -H "Content-Type: application/json" \
  "https://sgts.gitlab-dedicated.com/api/v4/projects/<encoded-project>/merge_requests/<MR_ID>/discussions/<discussion_id>/notes" \
  -d '{"body":"<reply>"}'
```

**GitHub:**
```bash
curl -s -X POST -H "Authorization: Bearer $GITHUB_TOKEN" -H "Content-Type: application/json" \
  "https://api.github.com/repos/<owner>/<repo>/pulls/<PR_ID>/comments" \
  -d '{"body":"<reply>","commit_id":"'"$HEAD_SHA"'","in_reply_to":<original_comment_id>}'
```

---

## RESOLVE

Mark thread as resolved.

**GitLab:**
```bash
curl -s -X PUT -H "PRIVATE-TOKEN: $GITLAB_TOKEN" -H "Content-Type: application/json" \
  "https://sgts.gitlab-dedicated.com/api/v4/projects/<encoded-project>/merge_requests/<MR_ID>/discussions/<discussion_id>" \
  -d '{"resolved":true}'
```

**GitHub (GraphQL):**
```bash
curl -s -X POST -H "Authorization: Bearer $GITHUB_TOKEN" -H "Content-Type: application/json" \
  "https://api.github.com/graphql" \
  -d '{"query":"mutation{resolveReviewThread(input:{threadId:\"<node_id>\"}){thread{isResolved}}}"}'
```
If node ID unavailable → REPLY with body `"Resolved."` and note limitation.

---

## APPROVE

Approve the MR/PR.

**GitLab:**
```bash
curl -s -X POST -H "PRIVATE-TOKEN: $GITLAB_TOKEN" \
  "https://sgts.gitlab-dedicated.com/api/v4/projects/<encoded-project>/merge_requests/<MR_ID>/approve"
```

**GitHub:**
```bash
curl -s -X POST -H "Authorization: Bearer $GITHUB_TOKEN" -H "Content-Type: application/json" \
  "https://api.github.com/repos/<owner>/<repo>/pulls/<PR_ID>/reviews" \
  -d '{"commit_id":"'"$HEAD_SHA"'","event":"APPROVE","body":"All threads addressed. LGTM!"}'
```
