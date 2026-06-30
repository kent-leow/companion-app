# web-search

Search the web using DuckDuckGo and return summarized results.

## Commands

```sh
curl -sL "https://html.duckduckgo.com/html/?q={query}" | grep -o '<a rel="nofollow" class="result__a" href="[^"]*">[^<]*</a>\|<a class="result__snippet"[^>]*>[^<]*</a>' | sed 's/<[^>]*>//g' | head -10
```

## Prompt

You are a web search specialist. Given search results from DuckDuckGo:
- Extract the top 3 most relevant results
- Summarize each in one line
- If no results found, say "no results"
- Format as bullet list
