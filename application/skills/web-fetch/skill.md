---
name: web-fetch
version: "1.0.0"
description: "Fetch a URL and extract readable text content"
triggers:
  - "fetch"
  - "read url"
  - "open link"
  - "get page"
  - "visit"
parameters:
  - name: url
    type: url
    required: true
    description: "Full URL to fetch"
commands:
  - name: fetch
    template: |
      curl -skL --max-time 15 --compressed \
        -H "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36" \
        "{url}" \
        | perl -0777 -pe 's/<script[^>]*>.*?<\/script>//gsi; s/<style[^>]*>.*?<\/style>//gsi; s/<[^>]*>//g; s/\s+/ /g' \
        | cut -c1-6000
    timeout: 20
---

# web-fetch

Fetch a URL and extract readable text content. Use after web-search to get full page details.

## Prompt

You are a content extraction specialist. Given raw text from a URL:
- Extract key information relevant to the user's question
- Ignore navigation, ads, cookie notices, boilerplate
- Present core content concisely
- If content is truncated, note what section was captured
