---
name: summarize
version: "1.0.0"
description: "Summarize long text, documents, or code into concise bullet points (LLM-only, no shell command)"
triggers:
  - "summarize"
  - "tldr"
  - "summary"
  - "brief"
  - "explain briefly"
parameters:
  - name: content
    type: string
    required: true
    description: "Text content to summarize"
  - name: max_bullets
    type: number
    required: false
    description: "Max bullet points (default: 5)"
commands: []
---

# summarize

Summarize the given content into bullet points (default 5). Focus on:
- Key facts and decisions
- Action items or implications
- Numbers, dates, names that matter

Skip boilerplate, introductions, and filler.
