# Plan: Companion Agent App

## Summary

Build a terminal-based AI agent CLI (like Claude Code / GitHub Copilot CLI) in **Node.js/TypeScript** that calls Claude models via **Platform AI gateway** (OpenAI-compatible API). The orchestrator follows a Claude Code-style agentic loop: receives user input → decides route → decomposes complex tasks into sub-agents with **minimal, focused context** → collects results concurrently → synthesizes a single response. Each sub-agent receives only what it needs (role prompt + task description + summarized conversation context), not the full history. The orchestrator maintains conversation continuity by summarizing prior turns and injecting that summary into each new LLM call.

**Key architectural choice**: The LLM outputs structured **action prefixes** (`[TOOL]`, `[SUBAGENT]`, `[RESPOND]`) that the orchestrator parses deterministically. This makes tool/skill invocation explicit and reliable — no regex guessing, no ambiguity. Skills follow a standardized `skill.md` schema (inspired by Claude's tool-use spec) with machine-readable metadata for discovery and invocation.

## Scope

### In

- TypeScript CLI binary (compiled via `tsx` / bundled with `esbuild`)
- Attractive interactive TUI (alien pixel art branding, streaming output)
- Platform AI gateway integration (OpenAI-compatible `/v1/chat/completions`, streaming SSE)
- Smart model switching: Haiku (simple) → Sonnet (default) → Opus (complex)
- **Claude Code-style orchestration loop:**
  - Router: classify intent (DIRECT / SEARCH / DECOMPOSE)
  - Planner: decompose into sub-agent specs with focused task descriptions
  - Sub-agent pool: execute concurrently with isolated, concise prompts
  - Synthesizer: merge sub-agent outputs into single concise response
  - **Context continuity**: each prompt carries a summary of prior conversation, not raw history
- **Structured action protocol** — LLM uses `[TOOL]`, `[SUBAGENT]`, `[RESPOND]` prefixes; orchestrator parses and executes deterministically
- **Standardized skill.md schema** — machine-readable frontmatter (name, description, triggers, parameters, commands) for orchestrator discovery
- `core.md` loaded every session as system context
- Skills system reading from `skills/<name>/skill.md`
- Web-search skill (DuckDuckGo HTML scraping, no API key)
- Local memory store (markdown-based, 8K token budget, self-pruning)
- Interactive input: paste text, paste images (multimodal), tag file/folder paths (`@path`)
- Multi-session support (session isolation for future chat integrations)
- Ultra-concise professional output formatting

### Out

- Electron desktop wrapper (separate effort)
- Telegram/Slack bot integrations (architecture supports it, implementation later)
- Landing page (separate package)

## Acceptance Criteria

| # | Criteria |
|---|----------|
| 1 | `companion` binary starts interactive terminal with alien pixel art splash + prompt |
| 2 | User input sent to Platform AI gateway (Sonnet default), streamed response displayed |
| 3 | Orchestrator decomposes multi-step queries into sub-tasks, dynamically creates role-prompted sub-agents |
| 4 | Sub-agents run concurrently via `Promise.all`, results collected and synthesized into single response |
| 5 | Model selection automatic: simple Q → Haiku, standard → Sonnet, complex reasoning → Opus |
| 6 | Sub-agents receive concise context only: `core.md` + role prompt + task + conversation summary (NOT full history) |
| 7 | Each orchestrator turn summarizes the previous assistant response and carries it forward as context for the next LLM call (chat flow continuity) |
| 8 | LLM responses use structured action prefixes (`[TOOL]`, `[SUBAGENT]`, `[RESPOND]`) — orchestrator parses and executes them deterministically |
| 9 | Skills follow standardized `skill.md` schema with YAML frontmatter (name, description, triggers, parameters, commands) |
| 10 | Orchestrator discovers and matches skills by searching frontmatter triggers/description — no hardcoded skill routing |
| 11 | Tool calls (`[TOOL]`) execute reliably: skill command run via `child_process`, output captured, fed back to LLM for next step |
| 12 | Skills loaded from `skills/<name>/skill.md` and invocable by orchestrator |
| 13 | Web-search skill (DuckDuckGo) performs search and returns summarized results |
| 14 | `core.md` loaded at session start and injected as system context for all LLM calls |
| 15 | Memory file read at start, updated during session, pruned to stay under 8K tokens |
| 16 | File/folder paths tagged in input (`@src/main.ts`) → content injected into context |
| 17 | Image paste supported in terminal input (base64 encoded, sent as multimodal message) |
| 18 | Multiple concurrent sessions supported (session ID isolation, shared config) |
| 19 | Responses are professional, ultra-concise (≤3 sentences default) |
| 20 | Startup time < 500ms |

## Open Questions

All resolved.

| # | Question | Resolution |
|---|----------|------------|
| ~~1~~ | AWS/auth credentials | Uses Platform AI gateway. Auth via `ANTHROPIC_BASE_URL` + `ANTHROPIC_AUTH_TOKEN` env vars |
| ~~2~~ | Memory token budget | 8K tokens |
| ~~3~~ | Web-search provider | DuckDuckGo (free, open-source, no API key needed) |
| ~~4~~ | Node.js vs Rust | Node.js/TypeScript — faster iteration, ecosystem maturity for CLI tooling, sufficient performance for I/O-bound agent work |
| ~~5~~ | How does LLM signal tool use? | Structured action prefixes (`[TOOL]`, `[SUBAGENT]`, `[RESPOND]`) in LLM output — parsed deterministically by orchestrator |
| ~~6~~ | Skill discovery mechanism | YAML frontmatter in `skill.md` with `triggers` array — orchestrator builds index at startup, matches by keyword/pattern |

## Estimate

- AC rows: 20
- Open Questions: 0
- Raw: (20 × 2) + 0 = 40
- **Story Points: 34** (nearest Fibonacci)
- **~68 days** (1 SP = 2 days)

## Folder Structure

```
application/
├── package.json
├── tsconfig.json
├── esbuild.config.ts           # Bundle to single binary-like JS
├── core.md                     # System instructions (loaded every session)
├── src/
│   ├── index.ts                # Entry, CLI setup, session init
│   ├── tui/
│   │   ├── index.ts            # Terminal UI module (ink or raw readline)
│   │   ├── input.ts            # Input: text, @-tags, image paste
│   │   ├── output.ts           # Streaming response rendering (marked-terminal)
│   │   └── splash.ts           # Alien pixel art startup screen
│   ├── orchestrator/
│   │   ├── index.ts            # Core orchestrator loop (route → plan → execute → synthesize)
│   │   ├── router.ts           # Intent classification (DIRECT/SEARCH/DECOMPOSE)
│   │   ├── planner.ts          # Task decomposition → sub-agent specs
│   │   ├── synthesizer.ts      # Merge sub-agent outputs → concise answer
│   │   ├── context-manager.ts  # Summarize prior turns, build concise context for each call
│   │   └── action-parser.ts    # Parse [TOOL], [SUBAGENT], [RESPOND] prefixes from LLM output
│   ├── agent/
│   │   ├── index.ts            # Sub-agent spawner & types
│   │   ├── pool.ts             # Concurrent agent execution (Promise.all + timeout)
│   │   └── prompt-builder.ts   # Role-based prompt construction + context injection
│   ├── llm/
│   │   ├── index.ts            # LLM client abstraction
│   │   ├── client.ts           # OpenAI-compatible API client (Platform AI gateway)
│   │   ├── streaming.ts        # SSE stream parser + token-by-token output
│   │   └── model-selector.ts   # Smart model routing (Haiku/Sonnet/Opus)
│   ├── skills/
│   │   ├── index.ts            # Skill loader + registry + search index
│   │   ├── loader.ts           # Parse skill.md YAML frontmatter + body → SkillDef
│   │   ├── executor.ts         # Execute skill commands (child_process spawn)
│   │   ├── matcher.ts          # Match user intent → skill via triggers/description
│   │   └── schema.ts           # TypeScript types for skill.md schema
│   ├── memory/
│   │   ├── index.ts            # Memory manager
│   │   ├── store.ts            # Read/write markdown memory
│   │   └── pruner.ts           # 8K token-aware pruning (tiktoken)
│   ├── session/
│   │   ├── index.ts            # Session manager (multi-session)
│   │   └── context.ts          # Per-session state isolation
│   └── config/
│       ├── index.ts            # Config loading (env vars, core.md)
│       └── env.ts              # Platform AI gateway config from env
├── skills/                         # All skills — default (no-config) + configurable
│   ├── web-search/
│   │   └── skill.md            # [DEFAULT] DuckDuckGo iterative search
│   ├── web-fetch/
│   │   └── skill.md            # [DEFAULT] Fetch URL + extract text
│   ├── read-file/
│   │   └── skill.md            # [DEFAULT] Read local file content
│   ├── run-command/
│   │   └── skill.md            # [DEFAULT] Execute shell commands
│   ├── summarize/
│   │   └── skill.md            # [DEFAULT] LLM-only summarization
│   ├── figma-design-context/
│   │   ├── skill.md            # [AUTH: FIGMA_TOKEN] Figma design extraction
│   │   └── scripts/            # get-metadata.sh, get-screenshot.sh, get-design-context.sh
│   ├── fix-vulnerabilities/
│   │   └── skill.md            # [AUTH: GITLAB_TOKEN] Security vulnerability reporter
│   ├── git-apis/
│   │   └── skill.md            # [AUTH: GITLAB_TOKEN/GITHUB_TOKEN] Git API operations
│   ├── git-workflow/
│   │   └── skill.md            # [AUTH: GITLAB_TOKEN/GITHUB_TOKEN] Branch/commit/push/MR
│   ├── gitlab-mr-automation/
│   │   └── skill.md            # [AUTH: GITLAB_TOKEN] Full MR lifecycle automation
│   └── jira-ticket/
│       ├── skill.md            # [AUTH: JIRA_*] Jira CRUD operations
│       └── scripts/            # create-ticket.sh, get-comments.sh, get-fields.sh
└── tests/
    ├── orchestrator.test.ts
    ├── action-parser.test.ts
    ├── skill-matcher.test.ts
    ├── client.test.ts
    ├── context-manager.test.ts
    └── memory.test.ts
```

## Tech Stack

| Component | Choice | Why |
|-----------|--------|-----|
| Language | TypeScript | Type safety, fast iteration, rich ecosystem for CLI/API tooling |
| Runtime | Node.js 20+ | LTS, native fetch, performant async I/O |
| CLI framework | commander + readline | Lightweight, standard |
| TUI rendering | chalk + ora + marked-terminal | Colored output, spinners, markdown rendering |
| HTTP client | native fetch (Node 20) | Zero-dep, streaming support via ReadableStream |
| YAML parser | yaml (npm) | Parse skill.md frontmatter |
| Tokenizer | tiktoken (via @dqbd/tiktoken) | Token counting for 8K memory pruning |
| Process exec | child_process (spawn) | Skill command execution |
| Testing | vitest | Fast, TypeScript-native, good DX |
| Bundling | esbuild | Fast bundling to single distributable JS |
| Image encoding | Buffer.from(...).toString('base64') | Native, zero-dep |

## LLM Integration Details

Reference: https://platform.ai.tech.gov.sg/models/#models-api-reference

```
Base URL:  https://api.ai.tech.gov.sg/platform/models
Auth:      x-api-key: $ANTHROPIC_AUTH_TOKEN
Format:    OpenAI-compatible (POST /v1/chat/completions)
Models:    bedrock.claude-haiku-4-5 | bedrock.claude-sonnet-4-6 | bedrock.claude-opus-4-6
Streaming: SSE (stream: true)
```

### Request Format (OpenAI-compatible)

```json
POST {BASE_URL}/v1/chat/completions
Headers:
  x-api-key: <key>
  Content-Type: application/json

Body:
{
  "model": "bedrock.claude-sonnet-4-6",
  "messages": [
    {"role": "system", "content": "...core.md content..."},
    {"role": "user", "content": "..."}
  ],
  "stream": true,
  "max_tokens": 4096
}
```

### Multimodal (Image)

```json
{
  "role": "user",
  "content": [
    {"type": "text", "text": "describe this"},
    {"type": "image_url", "image_url": {"url": "data:image/png;base64,..."}}
  ]
}
```

## Core Agent Instructions (`core.md`)

The `core.md` file is the **system prompt** that teaches the LLM how to behave and respond using the action protocol. This is what makes tool calls explicit and reliable — the LLM is instructed to ALWAYS use prefixes, never freeform.

### `core.md` Template (loaded into system prompt every call)

```markdown
# Core Instructions

## Identity
You are Companion — a concise, direct AI assistant with tool access.

## Communication
- Ultra concise, broken English ok
- 1-3 sentences default, no preambles ("Sure", "Great", etc.)
- Bullets/tables over prose; code over description
- Professional slang, TLDR style
- No hallucination — say "idk" over guessing

## Response Protocol

You MUST respond using EXACTLY ONE of these action prefixes. Never output bare text without a prefix.

### [RESPOND] — Answer the user directly
Use when you can answer from knowledge or after tool results are sufficient.
Format: `[RESPOND] <your answer>`

### [TOOL:<name>] — Call a tool/skill
Use when you need external data or action. The orchestrator will execute the tool and return the result.
Format: `[TOOL:<tool-name>] {"param": "value"}`
After receiving [TOOL_RESULT], continue reasoning and either call another tool or [RESPOND].

### [SUBAGENT:<role>] — Delegate to a specialist
Use for complex tasks needing parallel work. Emit multiple [SUBAGENT] lines — they run concurrently.
Format: `[SUBAGENT:<role-name>] {"task": "<focused task description>", "model": "<model_id>", "skill": "<optional-skill>"}`

### [THINK] — Internal reasoning (hidden from user)
Use for chain-of-thought before deciding action. Not shown to user.
Format: `[THINK] <reasoning>`

## Rules
1. ALWAYS start your response with a prefix. No bare text.
2. If you need info → [TOOL] first, then [RESPOND] after getting results.
3. If the task is complex (multi-step, multi-source) → [SUBAGENT] to decompose.
4. If you can answer directly → [RESPOND] immediately.
5. You may chain: [THINK] → [TOOL] → [TOOL] → [RESPOND] (multi-step is fine).
6. NEVER fabricate tool results. If a tool fails, say so.
7. NEVER claim you cannot search/access tools. You HAVE tool access.
8. Max 5 sub-agents per decomposition.

## Available Tools
{{TOOLS_SECTION}}
```

The `{{TOOLS_SECTION}}` placeholder is replaced at runtime by the orchestrator with the auto-generated tool list from the skill index (see "System Prompt: Available Tools Section" below).

### Sub-Agent `core.md` (Variant for Sub-Agents)

Sub-agents get a trimmed version — they can use `[TOOL]` and `[RESPOND]` but NOT `[SUBAGENT]` (no recursive spawning):

```markdown
# Sub-Agent Instructions

## Role: {{ROLE}}

## Communication
- Ultra concise — your output goes to a synthesizer, not a user
- Facts and data only, no filler
- Bullet format preferred

## Response Protocol

Use EXACTLY ONE prefix per line:
- [TOOL:<name>] {"param": "value"} — call a tool if needed
- [RESPOND] <your output> — your final answer (goes to synthesizer)
- [THINK] <reasoning> — internal reasoning

## Rules
1. ALWAYS prefix your output.
2. Focus ONLY on your assigned task.
3. You CANNOT spawn sub-agents.
4. After tool results, continue until you can [RESPOND].
5. Be factual. No hallucination.

## Context
{{CONTEXT_SUMMARY}}

## Available Tools
{{TOOLS_SECTION}}
```

## Structured Action Protocol

The LLM outputs structured action prefixes that the orchestrator parses deterministically. This replaces freeform text parsing and ensures tool/skill calls always execute correctly.

### Action Prefixes

| Prefix | Purpose | Orchestrator Behavior |
|--------|---------|----------------------|
| `[TOOL:<skill-name>]` | Invoke a skill/tool | Parse params → execute skill command → feed output back to LLM |
| `[SUBAGENT:<role>]` | Spawn a sub-agent | Create sub-agent with role + task → run concurrently |
| `[RESPOND]` | Final response to user | Stream text after prefix directly to terminal |
| `[THINK]` | Internal reasoning (not shown) | Store for context, don't display to user |

### LLM Output Format

```
[TOOL:web-search] {"query": "latest Node.js LTS version"}
```

```
[SUBAGENT:code-analyst] {"task": "Review the auth middleware for security issues", "model": "sonnet"}
[SUBAGENT:web-researcher] {"task": "Find OWASP top 10 for 2025", "model": "haiku", "skill": "web-search"}
```

```
[RESPOND] Node.js 22 is the current LTS. Released Oct 2025.
```

### Action Loop (Agentic Execution)

```
User Input
    ↓
LLM Call (with core.md + context summary + available tools)
    ↓
Parse output for action prefixes
    ↓
┌─────────────────────────────────────────────────────────────┐
│  [TOOL:*] → Execute skill command → capture output          │
│           → Append to messages as tool_result               │
│           → Call LLM again (loop continues)                 │
│                                                             │
│  [SUBAGENT:*] → Collect all sub-agent specs                 │
│              → Run concurrently (Promise.all)               │
│              → Feed results back → LLM synthesizes          │
│                                                             │
│  [RESPOND] → Stream to user → Turn complete                 │
│                                                             │
│  [THINK] → Store internally → Continue parsing              │
└─────────────────────────────────────────────────────────────┘
```

The loop continues until `[RESPOND]` is emitted. This allows multi-step tool use (search → fetch → analyze → respond) in a single turn — same pattern as Claude Code's agentic loop.

### System Prompt: Available Tools Section

At each LLM call, the orchestrator injects the available tools into the system prompt:

```
## Available Tools

You MUST use action prefixes to invoke tools. Output format:
[TOOL:<name>] {"param": "value", ...}

Available tools:
- web-search: Search the web via DuckDuckGo. Params: {"query": "<search terms>"}
- web-fetch: Fetch a URL and extract text. Params: {"url": "<url>"}
- git-apis: GitLab/GitHub API operations. Params: {"query": "<full request>"}
- figma-design-context: Extract Figma design specs. Params: {"query": "<figma URL or request>"}

When you need information or action, use [TOOL:name]. After receiving tool output, continue reasoning.
When ready to answer the user, use [RESPOND].
When spawning parallel work, use [SUBAGENT:role].
```

## Standardized Skill Schema (`skill.md`)

Every skill follows this schema. The YAML frontmatter is **machine-readable** — the orchestrator parses it at startup to build a searchable skill index.

### Schema Definition

```yaml
---
name: <string>                    # Unique identifier (kebab-case)
version: <string>                 # Semver
description: <string>             # One-line description (used for LLM tool descriptions)
triggers:                         # Keywords/patterns that activate this skill
  - <string>                      # e.g. "search", "look up", "find online"
  - <string>
parameters:                       # Input parameters the skill accepts
  - name: <string>                # Parameter name
    type: <string>                # string | number | boolean | url
    required: <boolean>
    description: <string>         # Shown to LLM as param description
auth:                             # Required credentials (checked at startup)
  - env: <string>                 # e.g. FIGMA_TOKEN
    description: <string>
commands:                         # Shell commands to execute
  - name: <string>                # Command identifier (e.g. "search", "fetch")
    template: <string>            # Shell command with {param} placeholders
    timeout: <number>             # Max execution time in seconds
---

# <Skill Name>

<Detailed prompt for the LLM when this skill is active>

## Usage Examples
...

## Commands (human-readable docs)
...
```

### Example: `web-search/skill.md`

```yaml
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
        | head -40
    timeout: 15
---

# web-search

Search the web using DuckDuckGo Lite and return structured results with URLs.

## Prompt

You are a web search specialist. Given search results (and optionally fetched page content):
- Synthesize the most relevant information into a direct, detailed answer
- Cite sources by mentioning the site name when attributing specific claims
- If results are insufficient, acknowledge and note the gap
- Concise bullet format preferred for multi-point answers
```

### Example: `git-apis/skill.md` (multi-command)

```yaml
---
name: git-apis
version: "1.0.0"
description: "GitLab + GitHub REST API: fetch discussions, post comments, reply, resolve threads, approve MR/PR"
triggers:
  - "gitlab"
  - "github"
  - "merge request"
  - "pull request"
  - "MR"
  - "PR"
  - "review comments"
  - "approve"
  - "resolve thread"
parameters:
  - name: query
    type: string
    required: true
    description: "Full user request including URLs and context"
auth:
  - env: GITLAB_TOKEN
    description: "GitLab personal access token"
  - env: GITHUB_TOKEN
    description: "GitHub personal access token"
commands:
  - name: preflight
    template: |
      echo "GitLab: $([ -n "$GITLAB_TOKEN" ] && echo OK || echo MISSING)"
      echo "GitHub: $([ -n "$GITHUB_TOKEN" ] && echo OK || echo MISSING)"
    timeout: 5
---

# git-apis
...
```

### Orchestrator Skill Index (Built at Startup)

```typescript
interface SkillIndex {
  skills: Map<string, SkillDef>;
  triggerMap: Map<string, string>;  // trigger keyword → skill name
}

// At startup:
// 1. Scan skills/ directory
// 2. Parse YAML frontmatter from each skill.md
// 3. Build triggerMap for fast lookup
// 4. Validate auth requirements (warn if env vars missing)
// 5. Generate "Available Tools" section for system prompt
```

## Orchestration Architecture (Claude Code-style)

```
User Input
    ↓
┌─────────────────────────────────────────────────────────────┐
│  ORCHESTRATOR LOOP                                          │
│                                                             │
│  1. Context Manager: summarize prior turns → "context blob" │
│     (keeps conversation flow without sending full history)  │
│                                                             │
│  2. Build system prompt:                                    │
│     core.md + available tools (from skill index) +          │
│     action prefix instructions                              │
│                                                             │
│  3. LLM Call (Sonnet):                                      │
│     system prompt + context summary + user message          │
│                                                             │
│  4. Parse response for action prefixes:                     │
│     [TOOL:*]     → execute skill → feed result back → loop │
│     [SUBAGENT:*] → collect specs → run parallel → synth    │
│     [RESPOND]    → stream to user → done                   │
│     [THINK]      → store, continue                         │
│                                                             │
│  5. Loop until [RESPOND] (max 10 iterations safety cap)     │
│                                                             │
└─────────────────────────────────────────────────────────────┘
    ↓
User Response (streamed)
    ↓
Context Manager: update rolling summary
```

### Sub-Agent Context Strategy (Key Differentiator)

Each sub-agent receives **minimal, focused context** — NOT the full conversation history:

```
┌─────────────────────────────────────────────────────────────┐
│  Sub-Agent Prompt Structure                                 │
│                                                             │
│  SYSTEM:                                                    │
│    - core.md (shared instructions)                          │
│    - Role definition ("You are a {role}")                   │
│    - Skill prompt (if skill-based agent)                    │
│    - Available tools for THIS agent (subset of all tools)   │
│    - Action prefix instructions                             │
│                                                             │
│  USER:                                                      │
│    - Context summary (1-3 sentences of prior conversation)  │
│    - Specific task description from planner                 │
│    - Any relevant data (URLs, code snippets)                │
│                                                             │
│  Sub-agent output format:                                   │
│    [TOOL:*] → orchestrator executes, feeds back             │
│    [RESPOND] → sub-agent's final output (goes to synth)    │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Context Continuity Flow

```
Turn 1:  User asks Q1
         → Orchestrator processes → Response R1
         → Context Manager creates summary: "User asked Q1. I answered R1."

Turn 2:  User asks Q2
         → Router receives: [summary of T1] + Q2
         → Planner receives: [summary of T1] + Q2
         → Sub-agents receive: [summary of T1 + T2 context] + focused task
         → Synthesizer merges → Response R2
         → Context Manager updates summary: "User asked Q1→R1, then Q2→R2"

Turn N:  Rolling summary (capped at ~2K tokens) ensures continuity
         without bloating sub-agent prompts
```

### Context Manager Implementation

```typescript
interface ConversationSummary {
  summary: string;       // Rolling summary of conversation (≤2K tokens)
  lastUserMessage: string;
  lastAssistantResponse: string;
  turnCount: number;
}

// After each turn, summarize:
// - If turnCount < 4: keep raw messages
// - If turnCount >= 4: LLM-summarize prior turns, keep last 2 raw
// - Always cap summary at 2K tokens
```

### Action Parser Implementation

```typescript
interface ParsedAction {
  type: 'TOOL' | 'SUBAGENT' | 'RESPOND' | 'THINK';
  target?: string;      // skill name or agent role
  params?: Record<string, unknown>;
  content?: string;     // for RESPOND/THINK
}

function parseActions(llmOutput: string): ParsedAction[] {
  const actions: ParsedAction[] = [];
  const lines = llmOutput.split('\n');

  for (const line of lines) {
    if (line.startsWith('[TOOL:')) {
      const match = line.match(/^\[TOOL:([^\]]+)\]\s*(.+)$/);
      if (match) {
        actions.push({
          type: 'TOOL',
          target: match[1],
          params: JSON.parse(match[2])
        });
      }
    } else if (line.startsWith('[SUBAGENT:')) {
      const match = line.match(/^\[SUBAGENT:([^\]]+)\]\s*(.+)$/);
      if (match) {
        actions.push({
          type: 'SUBAGENT',
          target: match[1],
          params: JSON.parse(match[2])
        });
      }
    } else if (line.startsWith('[RESPOND]')) {
      actions.push({
        type: 'RESPOND',
        content: line.slice('[RESPOND] '.length)
      });
    } else if (line.startsWith('[THINK]')) {
      actions.push({
        type: 'THINK',
        content: line.slice('[THINK] '.length)
      });
    }
  }
  return actions;
}
```

### Planner Output Format

```typescript
interface SubAgentSpec {
  role: string;          // e.g. "web researcher", "code analyst"
  task: string;          // Specific, focused task description
  modelHint: string;     // Which model to use
  skill?: string;        // Optional skill to invoke
  contextNeeded: string; // What context from conversation this agent needs
}
```

### Tool Execution Flow (Guaranteed Execution)

```typescript
async function executeToolAction(action: ParsedAction, skills: SkillIndex): Promise<string> {
  const skill = skills.get(action.target!);
  if (!skill) return `[ERROR] Unknown tool: ${action.target}`;

  // 1. Validate params against skill schema
  const validation = validateParams(action.params, skill.parameters);
  if (!validation.ok) return `[ERROR] Invalid params: ${validation.error}`;

  // 2. Build command from template + params
  const command = interpolateTemplate(skill.commands[0].template, action.params);

  // 3. Execute with timeout
  const { stdout, stderr, exitCode } = await execWithTimeout(
    command,
    skill.commands[0].timeout * 1000
  );

  // 4. Return structured result
  if (exitCode !== 0) return `[TOOL_ERROR] Exit ${exitCode}: ${stderr}`;
  return `[TOOL_RESULT:${action.target}]\n${stdout}`;
}
```

## Default Skills (Shipped Out-of-Box)

These skills ship with the app and require NO configuration. They form the core toolchain that the orchestrator always has available.

### 1. `web-search` — Iterative Web Search

**Key behavior**: Searches iteratively until useful results are retrieved. If first query returns poor results, the orchestrator refines the query and retries (max 3 attempts).

```yaml
---
name: web-search
version: "1.0.0"
description: "Search the web using DuckDuckGo. Iterates with refined queries until results are found (max 3 attempts)."
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
retry:
  max_attempts: 3
  on_empty: "refine_query"
---

# web-search

## Prompt

You are a web search specialist. Search iteratively until you find relevant results.

## Iteration Protocol

1. Execute search with initial query
2. If [TOOL_RESULT] is empty or irrelevant:
   - [THINK] Analyze why results were poor
   - [TOOL:web-search] {"query": "<refined query with different keywords>"}
3. Repeat up to 3 times with progressively different search strategies:
   - Attempt 1: direct query
   - Attempt 2: rephrase with synonyms / more specific terms
   - Attempt 3: broaden or use alternative angle
4. After getting results, synthesize:
   - Cite sources by site name
   - Bullet format for multi-point answers
   - If still insufficient after 3 attempts, say what was found and note the gap

## Example Flow

```
[TOOL:web-search] {"query": "Node.js 22 LTS release date"}
→ [TOOL_RESULT:web-search] (empty or irrelevant)
[THINK] No results for exact query. Try broader terms.
[TOOL:web-search] {"query": "Node.js LTS 2025 release schedule"}
→ [TOOL_RESULT:web-search] URL: ... TITLE: ... SNIPPET: ...
[RESPOND] Node.js 22 entered LTS on Oct 29, 2024 (source: nodejs.org).
```
```

### 2. `web-fetch` — Fetch & Extract URL Content

```yaml
---
name: web-fetch
version: "1.0.0"
description: "Fetch a URL and extract readable text content. Use after web-search to get full page details."
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

## Prompt

You are a content extraction specialist. Given raw text from a URL:
- Extract key information relevant to the user's question
- Ignore navigation, ads, cookie notices, boilerplate
- Present core content concisely
- If content is truncated, note what section was captured
```

### 3. `read-file` — Read Local File Content

```yaml
---
name: read-file
version: "1.0.0"
description: "Read a local file's content. Supports text files, code, config, markdown."
triggers:
  - "read file"
  - "show file"
  - "cat"
  - "open file"
  - "@"
parameters:
  - name: path
    type: string
    required: true
    description: "Absolute or relative file path"
  - name: lines
    type: number
    required: false
    description: "Max lines to read (default: 200)"
commands:
  - name: read
    template: |
      if [ -f "{path}" ]; then
        head -n ${lines:-200} "{path}"
      else
        echo "[ERROR] File not found: {path}"
      fi
    timeout: 5
---

# read-file

## Prompt

Read and return file contents. Used when user references a file with @path or asks to look at code.
```

### 4. `run-command` — Execute Shell Command

```yaml
---
name: run-command
version: "1.0.0"
description: "Execute a shell command and return stdout/stderr. Use for builds, tests, installs, git status."
triggers:
  - "run"
  - "execute"
  - "shell"
  - "terminal"
  - "npm"
  - "git status"
  - "build"
  - "test"
  - "install"
parameters:
  - name: command
    type: string
    required: true
    description: "Shell command to execute"
  - name: cwd
    type: string
    required: false
    description: "Working directory (default: session cwd)"
commands:
  - name: exec
    template: |
      cd "{cwd:-.}" && {command} 2>&1 | head -100
    timeout: 30
---

# run-command

## Prompt

Execute shell commands. Return stdout/stderr truncated to 100 lines.
Safety: never execute commands that delete data, modify system config, or expose secrets unless explicitly confirmed.
```

### 5. `summarize` — Summarize Long Content

```yaml
---
name: summarize
version: "1.0.0"
description: "Summarize long text, documents, or code into concise bullet points. No shell command — LLM-only skill."
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

## Prompt

Summarize the given content into ≤5 bullet points (or specified max). Focus on:
- Key facts and decisions
- Action items or implications
- Numbers, dates, names that matter
Skip boilerplate, introductions, and filler.
```

### 6. `figma-design-context` — Extract Figma Design Specs

```yaml
---
name: figma-design-context
version: "1.0.0"
description: "Extract Figma design context via REST API: layout, typography, colors, UI flow, components, variables, comments, assets."
triggers:
  - "figma"
  - "figma.com"
  - "design"
  - "design spec"
  - "UI spec"
  - "mockup"
  - "prototype"
parameters:
  - name: query
    type: string
    required: true
    description: "Figma URL or file key with optional node-id, or description of what to extract"
auth:
  - env: FIGMA_TOKEN
    description: "Figma personal access token (Settings → Personal Access Tokens)"
commands:
  - name: preflight
    template: |
      echo "FIGMA_TOKEN: $([ -n "$FIGMA_TOKEN" ] && echo OK || echo MISSING)"
    timeout: 5
  - name: get-metadata
    template: |
      bash application/skills/figma-design-context/scripts/get-metadata.sh --file-key {fileKey}
    timeout: 15
  - name: get-screenshot
    template: |
      bash application/skills/figma-design-context/scripts/get-screenshot.sh \
        --file-key {fileKey} --node-id {nodeId} --scale 2 --output ./figma-screenshot.png
    timeout: 30
  - name: get-design-context
    template: |
      bash application/skills/figma-design-context/scripts/get-design-context.sh \
        --file-key {fileKey} --node-id {nodeId} --output ./figma-context.json
    timeout: 30
---

# figma-design-context

## Prompt

You are a Figma design extraction specialist. Given a Figma URL or file key, use the scripts to extract design context.

## URL Parsing

`https://www.figma.com/design/AbCdEfGhIj/My-Project?node-id=2313-102848`
- `fileKey` = `AbCdEfGhIj` (segment after `/design/`)
- `nodeId` = `2313:102848` (query param `node-id`, convert `-` → `:`)

## Standard Workflow

1. Parse Figma URL → extract fileKey + nodeId
2. [TOOL:figma-design-context] with `get-metadata` → discover pages/frames
3. [TOOL:figma-design-context] with `get-screenshot` → visual reference
4. [TOOL:figma-design-context] with `get-design-context` → full spec JSON
5. [RESPOND] with structured design context (layout, colors, typography, spacing)
```

### 7. `fix-vulnerabilities` — Security Vulnerability Reporter

```yaml
---
name: fix-vulnerabilities
version: "1.0.0"
description: "Fetch and report GitLab security vulnerabilities (critical/high/medium/low) for repos. Report only — does NOT apply fixes."
triggers:
  - "vulnerability"
  - "vulnerabilities"
  - "security scan"
  - "CVE"
  - "security report"
  - "dependency scan"
parameters:
  - name: query
    type: string
    required: true
    description: "GitLab MR URL or repo name to scan for vulnerabilities"
auth:
  - env: GITLAB_TOKEN
    description: "GitLab token with read_api scope"
commands:
  - name: preflight
    template: |
      echo "GITLAB_TOKEN: $([ -n "$GITLAB_TOKEN" ] && echo OK || echo MISSING)"
    timeout: 5
  - name: fetch-vulnerabilities
    template: |
      tmpfile=$(/usr/bin/mktemp)
      all_results="[]"
      for SEV in critical high medium low; do
        page=1
        while true; do
          chunk=$(/usr/bin/curl -s -H "PRIVATE-TOKEN: $GITLAB_TOKEN" \
            "https://sgts.gitlab-dedicated.com/api/v4/projects/{project_path}/vulnerability_findings?severity[]=${SEV}&per_page=100&page=${page}")
          count=$(echo "$chunk" | /usr/bin/jq 'length')
          [ "$count" -eq 0 ] && break
          all_results=$(echo "$all_results $chunk" | /usr/bin/jq -s '.[0] + .[1]')
          page=$((page + 1))
          [ $page -gt 10 ] && break
        done
      done
      echo "$all_results" | /usr/bin/jq '[.[] | select(.state == "detected") | {
        id, name, severity, state,
        scanner: .scanner.name,
        file: .location.file,
        dependency_pkg: .location.dependency.package.name,
        dependency_ver: .location.dependency.version,
        solution, identifiers: [.identifiers[].name]
      }] | group_by(.severity) | map({severity: .[0].severity, count: length, findings: .})'
      /bin/rm -f "$tmpfile"
    timeout: 60
---

# fix-vulnerabilities

## Prompt

You are a security vulnerability reporter. Given a GitLab MR URL or repo name:
1. Extract project path from URL, URL-encode it
2. Run preflight to verify token
3. Fetch all vulnerability findings across all severities
4. Report structured summary: CRITICAL/HIGH/MEDIUM/LOW counts + details
5. STOP: Do NOT apply fixes, run tests, or execute git commands
```

### 8. `git-apis` — Git Platform API Operations

```yaml
---
name: git-apis
version: "1.0.0"
description: "GitLab + GitHub REST API: fetch discussions, post inline/general comments, reply, resolve threads, approve MR/PR."
triggers:
  - "gitlab"
  - "github"
  - "merge request"
  - "pull request"
  - "MR"
  - "PR"
  - "review comments"
  - "approve"
  - "resolve thread"
  - "post comment"
  - "discussions"
parameters:
  - name: query
    type: string
    required: true
    description: "Full user request including URLs and context"
auth:
  - env: GITLAB_TOKEN
    description: "GitLab personal access token"
  - env: GITHUB_TOKEN
    description: "GitHub personal access token"
commands:
  - name: preflight
    template: |
      echo "GitLab: $([ -n "$GITLAB_TOKEN" ] && echo OK || echo MISSING)"
      echo "GitHub: $([ -n "$GITHUB_TOKEN" ] && echo OK || echo MISSING)"
    timeout: 5
---

# git-apis

## Prompt

You are a Git platform API specialist. Execute the appropriate API operation:

## Operations

| Operation | Description |
|-----------|-------------|
| FETCH_DISCUSSIONS | Get all MR/PR discussions (paginated) |
| POST_INLINE | Post inline comment on a diff line |
| POST_GENERAL | Post general (non-inline) comment |
| REPLY | Reply to an existing thread |
| RESOLVE | Mark thread as resolved |
| APPROVE | Approve the MR/PR |

## Auth Headers

| Platform | Header |
|---|---|
| GitLab | `PRIVATE-TOKEN: $GITLAB_TOKEN` |
| GitHub | `Authorization: Bearer $GITHUB_TOKEN` |

Use [TOOL:run-command] to execute curl commands for each operation. Parse URLs to extract project path, MR/PR ID.
```

### 9. `git-workflow` — Git Branch/Commit/Push/MR Lifecycle

```yaml
---
name: git-workflow
version: "1.0.0"
description: "Git workflow automation: branch setup, commit, push, MR creation, pipeline polling, review-thread lifecycle. Supports GitLab and GitHub."
triggers:
  - "branch"
  - "commit"
  - "push"
  - "create MR"
  - "create PR"
  - "merge request"
  - "pipeline"
  - "CI"
  - "review fix"
parameters:
  - name: query
    type: string
    required: true
    description: "Workflow request (e.g. 'create branch and MR for ticket PROJ-123')"
auth:
  - env: GITLAB_TOKEN
    description: "GitLab personal access token"
  - env: GITHUB_TOKEN
    description: "GitHub personal access token"
commands:
  - name: preflight
    template: |
      echo "GitLab: $([ -n "$GITLAB_TOKEN" ] && echo OK || echo MISSING)"
      echo "GitHub: $([ -n "$GITHUB_TOKEN" ] && echo OK || echo MISSING)"
    timeout: 5
---

# git-workflow

## Prompt

You are a Git workflow orchestrator. Execute the appropriate workflow phase:

## Phases

| Phase | Description |
|-------|-------------|
| BRANCH_SETUP | Resolve ticket, create/checkout branch, sync |
| COMMIT | Stage + commit with conventional message |
| PUSH | Push to remote (never force-push) |
| ENSURE_MR | Create or find existing MR/PR |
| POLL_PIPELINE | Adaptive polling until success/failure/timeout |
| FETCH_OPEN_THREADS | Get unresolved review threads |
| POST_THREAD_REPLIES | Reply to fixed/rejected threads |
| RESOLVE_THREADS | Resolve addressed threads |

## Constraints

- Never force-push
- Never commit secrets/tokens
- Never auto-approve or auto-merge
- Use absolute paths: `/usr/bin/curl`, `/usr/bin/jq`, `/usr/bin/git`
- Run full workflow to completion without pausing

## Branch Naming

| Context | Pattern |
|---|---|
| Task implementation | `GOBIZWKST2-{TICKET}-{kebab-task-title}` |
| Vulnerability fixes | `GOBIZWKST2-{TICKET}-Fix-Vulnerability-{YYYYMMDD}` |
| Hotfix | `GOBIZWKST2-{TICKET}-hotfix-{description}` |
```

### 10. `gitlab-mr-automation` — Full MR Lifecycle Automation

```yaml
---
name: gitlab-mr-automation
version: "1.0.0"
description: "Self-contained GitLab MR automation: branch → commit → push → MR → poll pipeline → resolve threads. Full lifecycle, runs to completion."
triggers:
  - "automate MR"
  - "submit code"
  - "implement task"
  - "fix review"
  - "resolve threads"
  - "pipeline fix"
  - "MR lifecycle"
parameters:
  - name: query
    type: string
    required: true
    description: "Full request with repo dir, branch pattern, commit msg, MR title"
auth:
  - env: GITLAB_TOKEN
    description: "GitLab personal access token"
commands:
  - name: preflight
    template: |
      echo "GITLAB_TOKEN: $([ -n "$GITLAB_TOKEN" ] && echo OK || echo MISSING)"
    timeout: 5
---

# gitlab-mr-automation

## Prompt

You are a GitLab MR automation agent. Execute the full lifecycle:
ticket → branch → code → commit → push → MR → poll until pipeline=success AND open_threads=0.

## Input

| Input | Required | Description |
|---|---|---|
| REPO_DIR | Yes | Absolute path to repo |
| BRANCH_PATTERN | Yes | Pattern with `{TICKET}` placeholder |
| COMMIT_MSG | Yes | Initial commit message |
| MR_TITLE | Yes | Merge request title |

## Terminal States

| Condition | Status |
|---|---|
| Pipeline success + 0 open threads | SUCCESS |
| 3 consecutive pipeline failures | BLOCKED |
| 20 polls exceeded | TIMEOUT |

## Constraints

- Never force-push
- Never commit secrets
- Never auto-approve or auto-merge
- Post thread replies BEFORE pipeline wait
- Run full loop to completion — no user prompts mid-workflow
```

### 11. `jira-ticket` — Jira Issue Management

```yaml
---
name: jira-ticket
version: "1.0.0"
description: "Create/retrieve Jira issues: tickets, sub-tasks, story points, comments. Full CRUD via REST API."
triggers:
  - "jira"
  - "ticket"
  - "story"
  - "sub-task"
  - "story points"
  - "atlassian"
  - "create ticket"
  - "issue"
parameters:
  - name: query
    type: string
    required: true
    description: "Jira request (e.g. 'create story for auth refactor, 5 SP')"
auth:
  - env: JIRA_TOKEN
    description: "Jira API token"
  - env: JIRA_BASE_URL
    description: "e.g. https://your-org.atlassian.net"
  - env: JIRA_PROJECT_KEY
    description: "e.g. PROJ"
  - env: JIRA_EMAIL
    description: "Atlassian account email"
commands:
  - name: preflight
    template: |
      echo "JIRA_TOKEN: $([ -n "$JIRA_TOKEN" ] && echo OK || echo MISSING)"
      echo "JIRA_BASE_URL: $([ -n "$JIRA_BASE_URL" ] && echo OK || echo MISSING)"
      echo "JIRA_PROJECT_KEY: $([ -n "$JIRA_PROJECT_KEY" ] && echo OK || echo MISSING)"
      echo "JIRA_EMAIL: $([ -n "$JIRA_EMAIL" ] && echo OK || echo MISSING)"
    timeout: 5
  - name: create-ticket
    template: |
      bash application/skills/jira-ticket/scripts/create-ticket.sh \
        --title "{title}" --description "{description}" \
        --issue-type "{issueType}" --story-points {storyPoints}
    timeout: 15
  - name: get-comments
    template: |
      bash application/skills/jira-ticket/scripts/get-comments.sh \
        --issue-key "{issueKey}" --max-results 20
    timeout: 10
---

# jira-ticket

## Prompt

You are a Jira ticket management specialist. Given a user request:

## Operations

| Operation | Script |
|-----------|--------|
| Create ticket | `create-ticket.sh --title --description --issue-type --story-points` |
| Create sub-task | `create-ticket.sh --title --description --issue-type Sub-task --parent PROJ-123` |
| Update story points | `update-story-points.sh --issue-key --story-points` |
| Get comments | `get-comments.sh --issue-key [--max-results N]` |
| Discover fields | `get-fields.sh` (use customfield_10274 for story points) |

## Workflow

1. Parse user request → determine operation
2. Run preflight → verify all env vars present
3. Execute appropriate script
4. Persist state to `.docs/<task>/jira.json`
5. Report: issue key + URL + story points
```

### Iterative Tool Loop Pattern (Built Into Orchestrator)

The orchestrator's action loop inherently supports iterative tool calls. The LLM can call the same tool multiple times with different params — the loop continues until `[RESPOND]`:

```typescript
async function agentLoop(messages: ChatMessage[], skills: SkillIndex): Promise<string> {
  let iterations = 0;
  const MAX_ITERATIONS = 10;

  while (iterations < MAX_ITERATIONS) {
    iterations++;
    const response = await llm.chat(messages);
    const actions = parseActions(response);

    for (const action of actions) {
      switch (action.type) {
        case 'RESPOND':
          return action.content!;

        case 'TOOL': {
          const result = await executeToolAction(action, skills);
          // Feed result back into messages — LLM sees it next iteration
          messages.push({ role: 'assistant', content: response });
          messages.push({ role: 'user', content: result });
          break;
        }

        case 'SUBAGENT': {
          // Collect all SUBAGENT actions, run in parallel
          const specs = actions.filter(a => a.type === 'SUBAGENT');
          const results = await runSubAgentsParallel(specs, skills);
          messages.push({ role: 'assistant', content: response });
          messages.push({ role: 'user', content: formatSubAgentResults(results) });
          break;
        }

        case 'THINK':
          // Store but don't feed back — it's already in the response
          break;
      }
    }

    // If no RESPOND found, loop continues (LLM gets tool results and decides next step)
  }

  return '[ERROR] Max iterations reached without response';
}
```

This means **web-search iterates naturally** — the LLM sees empty results, thinks about why, and calls `[TOOL:web-search]` again with a better query. No special retry logic needed in the skill itself — the orchestrator loop handles it.

### Skill Dependency Chain (Neighbour Pattern)

Skills can reference each other. The orchestrator supports **chained tool calls** in a single turn:

```
User: "What's the latest on Rust async traits?"

LLM:
[TOOL:web-search] {"query": "Rust async traits stabilization 2025"}
→ [TOOL_RESULT] URL: https://blog.rust-lang.org/... SNIPPET: "async fn in traits..."
[THINK] Found a blog post. Need full details.
[TOOL:web-fetch] {"url": "https://blog.rust-lang.org/2025/03/async-traits.html"}
→ [TOOL_RESULT] Full article text...
[RESPOND] Async traits stabilized in Rust 1.78 (March 2025). Key points: ...
```

Common chains:
| Pattern | Flow |
|---------|------|
| Search + Fetch | `web-search` → find URL → `web-fetch` → extract details → respond |
| Search + Iterate | `web-search` → poor results → `web-search` (refined) → respond |
| Read + Summarize | `read-file` → long content → `summarize` → respond |
| Command + Analyze | `run-command` → output → think → respond with analysis |

## Notes

- **Platform AI gateway** — OpenAI-compatible interface, auth via `x-api-key` header. Env vars already configured.
- **Action prefixes are deterministic** — no fuzzy matching. If the LLM outputs `[TOOL:web-search]`, the orchestrator will always execute `web-search`. If the prefix is malformed, the orchestrator asks the LLM to retry (max 2 retries).
- **Skill discovery** — at startup, orchestrator scans `skills/` directory, parses all YAML frontmatter, builds trigger index. The "Available Tools" section in the system prompt is auto-generated from this index.
- **Tool execution is guaranteed** — once parsed, tools always execute. Failures are captured and fed back to the LLM as `[TOOL_ERROR]` for recovery.
- **Iterative search** — web-search naturally iterates via the action loop. LLM sees empty results, refines query, calls again. Max 3 search attempts per turn (enforced by LLM instructions in skill prompt + 10-iteration safety cap).
- **Default skills require zero config** — web-search, web-fetch, read-file, run-command, summarize all work out-of-box with no API keys or tokens.
- **Context efficiency** — sub-agents get 50-200 tokens of context summary, not 4K+ of raw history.
- **Conversation continuity** — the context manager summarizes after each turn. If summary exceeds 2K tokens, it re-summarizes itself.
- **Safety cap** — the action loop runs max 10 iterations per turn to prevent infinite tool loops.
- Sub-agents are ephemeral — spawned per-request, no persistent state.
- Multi-session: each session gets an ID, isolated memory + conversation state.
- Image paste: base64 encode + send as multimodal content part.
- Alien pixel art: ASCII/braille art rendered at startup via chalk.

## Changelog

| Date | Change |
|------|--------|
| 2026-07-01 | Initial plan created from raw requirements |
| 2026-07-01 | Refined: dynamic sub-agents, core.md, image paste, multi-session, alien TUI |
| 2026-07-01 | Added Platform AI API reference |
| 2026-07-01 | **Major revision**: Rust → Node.js/TypeScript. Added Claude Code-style orchestration with context manager (summarize-and-forward pattern). Sub-agents receive minimal focused context, not full history. Each prompt continues chat flow via rolling conversation summary. |
| 2026-07-01 | **Action protocol**: Added structured action prefixes (`[TOOL]`, `[SUBAGENT]`, `[RESPOND]`, `[THINK]`). Standardized `skill.md` schema with YAML frontmatter for machine-readable discovery. Added `action-parser.ts`, `matcher.ts`, `schema.ts`. Tool execution is now deterministic and guaranteed. Orchestrator builds skill index at startup from frontmatter triggers/description. |
| 2026-07-01 | **Default skills**: Added 5 zero-config skills (web-search, web-fetch, read-file, run-command, summarize) + 6 auth-required skills (figma-design-context, fix-vulnerabilities, git-apis, git-workflow, gitlab-mr-automation, jira-ticket). Total 11 skills. Web-search uses iterative pattern. All skills follow standardized YAML frontmatter schema. |
