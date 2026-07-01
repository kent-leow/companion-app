---
name: another-skill
version: "2.0.0"
description: "Another test skill"
triggers:
  - "search"
  - "find"
parameters:
  - name: query
    type: string
    required: true
    description: "Search query"
commands:
  - name: search
    template: |
      echo "Searching for {query}"
    timeout: 10
---

# another-skill

Another test skill for registry tests.
