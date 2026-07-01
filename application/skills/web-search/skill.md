# web-search

Search the web using DuckDuckGo Lite and return summarized results.

## Commands

```sh
curl -sk --max-time 10 --compressed --get --data-urlencode "q={query}" -H "User-Agent: Lynx/2.9.2 libwww-FM/2.14" "https://lite.duckduckgo.com/lite/" | tr -d '\r' | sed -n "/class='result-link'/{ s/.*class='result-link'>//; s/<\/a.*//; p; }; /class='result-snippet'/{ n; s/^[[:space:]]*//; s/<[^>]*>//g; p; }" | head -10
```

## Prompt

You are a web search specialist. Given search results from DuckDuckGo:
- Synthesize the top 3 most relevant results into a direct answer
- If results are empty, tell the user the search could not retrieve results and suggest they check manually
- Be concise, bullet list format
