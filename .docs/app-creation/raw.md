1. Create a companion agent app that help to automate workflow/development/answering. Bascially a clean clone of Claude Code/GHCP/etc but from scratch.
2. The agent app should use the best programming language (fast and efficient) that make the agent most responsive.
3. The agent app should call to Bedrock Claude Code with the token. And able to switch smartly between different models depends on the task.
4. The agent app should have a ochestrator LLM call which manage core context and divide the task into multiple sub-tasks, then asynchrously spin up several sub-agent(LLM call) to do fetch/reasoning/coding/debugging/logging/db searching/web search etc(all use skill to achieve). Then once all required responses are collected, do pass back to core ochestrator to facilitate and do answering.
5. Similarily, there're common `skills` concept, which reads from `/skills/xxx/skill.md`
6. The instruction-wise (must load every start of chat/agent load), refer to CLAUDE.MD. Ensure all response is utlra short/clean/concise.
7. By default, give me web-search skill first. We will add-on later.
8. This companion agent by default should be terminal-style like Claude Code, GitHub Copilot CLI, etc. Make it as interactive as mentioned. Able to paste image, tag file/folder path, etc. But it doesn't need to show the code change etc, it's just a simple chat interface like slacks/ws/telegram/etc. And yes, it will be integrate to those afterward(like telegram bot/slacks bot/etc), but not now.
9. This companion is a profesional bot using professional slang but ultra concise with short TLDR yet includes all required context/content.
10. This companion app should keep the memory locally and able to improve it while it's running. So the context md will be always updated but keep within certain length, so it evolves without causing more token.
11. More to be explore, please search online and give me more suggestions and improvements etc.