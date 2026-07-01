---
name: web-search
version: "1.0.0"
description: "Search the web using DuckDuckGo and return structured results with URLs and snippets"
triggers:
  - "search"
  - "look up"
  - "find online"
  - "google"
  - "what is"
  - "latest"
  - "current"
  - "news"
  - "recent"
  - "who is"
  - "when did"
  - "how to"
parameters:
  - name: query
    type: string
    required: true
    description: "Search query terms"
commands:
  - name: search
    template: |
      curl -sk --max-time 10 --compressed --get \
        --data-urlencode "q={query}" \
        -H "User-Agent: Lynx/2.9.2 libwww-FM/2.14" \
        "https://lite.duckduckgo.com/lite/" \
        | tr -d '\r' \
        | perl -ne 'if(/uddg=([^&"]+)/){$u=$1;$u=~s/%([0-9A-Fa-f]{2})/chr(hex($1))/ge;print"URL: $u\n"}if(/result-link.>([^<]+)/){print"TITLE: $1\n"}if(/result-snippet/){$s=<>;$s=~s/^\s+//;$s=~s/<[^>]*>//g;chomp$s;print"SNIPPET: $s\n---\n"}' \
        | head -60
    timeout: 15
---

# web-search

Search the web using DuckDuckGo Lite and return structured results with URLs.

## Prompt

You are a web search specialist. Search iteratively until you find relevant results.

## Iteration Protocol

1. Execute search with initial query
2. If [TOOL_RESULT] is empty or irrelevant:
   - [THINK] Analyze why results were poor
   - [TOOL:web-search] {"query": "<refined query>"}
3. Repeat up to 3 times with different strategies:
   - Attempt 1: direct query
   - Attempt 2: rephrase with synonyms
   - Attempt 3: broaden or use alternative angle
4. After getting results, synthesize:
   - Cite sources by site name
   - Bullet format for multi-point answers
   - If still insufficient after 3 attempts, note the gap
