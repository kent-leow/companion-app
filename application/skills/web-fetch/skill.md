# web-fetch

Fetch a URL and extract readable text content from the page.

## Commands

```sh
curl -skL --max-time 15 --compressed -H "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36" "{query}" | perl -0777 -pe 's/<script[^>]*>.*?<\/script>//gsi; s/<style[^>]*>.*?<\/style>//gsi; s/<[^>]*>//g; s/\s+/ /g' | cut -c1-4000
```

## Prompt

You are a content extraction specialist. Given the raw text content fetched from a URL:
- Extract the key information relevant to the user's original question
- Ignore navigation, ads, cookie notices, and boilerplate
- Present the core content concisely
