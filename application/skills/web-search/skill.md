# web-search

Search the web using DuckDuckGo and return summarized results.

## Commands

```sh
python3 -c "
import urllib.request, urllib.parse, json, re, ssl, sys
q = urllib.parse.quote_plus('{query}')
ctx = ssl.create_default_context()
ctx.check_hostname = False
ctx.verify_mode = ssl.CERT_NONE
# Try instant answer API
try:
    req = urllib.request.Request(
        f'https://api.duckduckgo.com/?q={q}&format=json&no_html=1&skip_disambig=1',
        headers={'User-Agent': 'companion/0.1'})
    data = json.loads(urllib.request.urlopen(req, timeout=8, context=ctx).read())
    parts = []
    if data.get('AbstractText'): parts.append(data['AbstractText'])
    for t in data.get('RelatedTopics', [])[:5]:
        if isinstance(t, dict) and t.get('Text'): parts.append(t['Text'])
    if parts:
        print('\n'.join(parts))
        sys.exit(0)
except Exception:
    pass
# Try HTML search
try:
    req = urllib.request.Request(
        f'https://html.duckduckgo.com/html/?q={q}',
        headers={'User-Agent': 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
                 'Accept': 'text/html', 'Accept-Language': 'en-US,en;q=0.9'})
    html = urllib.request.urlopen(req, timeout=10, context=ctx).read().decode('utf-8', errors='ignore')
    if 'botnet' not in html and 'captcha' not in html.lower():
        snippets = re.findall(r'class=\"result__snippet\"[^>]*>(.*?)</a>', html)
        titles = re.findall(r'class=\"result__a\"[^>]*>(.*?)</a>', html)
        if titles:
            for t, s in zip(titles[:5], snippets[:5]):
                print(re.sub(r'<[^>]+>', '', t) + ': ' + re.sub(r'<[^>]+>', '', s))
            sys.exit(0)
except Exception:
    pass
print('ERROR: web search unavailable (network blocked or rate-limited)')
"
```

## Prompt

You are a web search specialist. Given search results from DuckDuckGo:
- Extract the top 3 most relevant results
- Summarize each in one line
- If results say "ERROR" or are empty, tell the user the search failed and suggest they check manually
- Format as bullet list
