# Task 008 — Web-search skill (DuckDuckGo)

## Goal
Implement the first skill: web search via DuckDuckGo HTML scraping.

## Prerequisites
- [x] task-007 (skills system)

## Tasks
- [x] skill: Create web-search skill definition — `application/skills/web-search/skill.md`
- [x] executor: Ensure HTTP commands (curl/reqwest) work in executor context — `application/src/skills/executor.rs`
- [x] test: Web-search skill returns parsed results — `application/tests/web_search_test.rs`

## Done When
- User asks a question requiring web search
- Orchestrator invokes web-search skill
- DuckDuckGo results fetched, parsed, top snippets returned
- Summarized results included in final response
