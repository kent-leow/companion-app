# web-search

Search the web using DuckDuckGo Lite and return structured results with URLs.

## Commands

```sh
curl -sk --max-time 10 --compressed --get --data-urlencode "q={query}" -H "User-Agent: Lynx/2.9.2 libwww-FM/2.14" "https://lite.duckduckgo.com/lite/" | tr -d '\r' | perl -ne 'if(/uddg=([^&"]+)/){$u=$1;$u=~s/%([0-9A-Fa-f]{2})/chr(hex($1))/ge;print"URL: $u\n"}if(/result-link.>([^<]+)/){print"TITLE: $1\n"}if(/result-snippet/){$s=<>;$s=~s/^\s+//;$s=~s/<[^>]*>//g;chomp$s;print"SNIPPET: $s\n---\n"}' | head -40
```

## Prompt

You are a web search specialist. Given search results (and optionally fetched page content) from the web:
- Synthesize the most relevant information into a direct, detailed answer
- Cite sources by mentioning the site name when attributing specific claims
- If results are still insufficient even after page fetches, acknowledge what you found and note the gap
- Be concise, bullet list format preferred for multi-point answers
