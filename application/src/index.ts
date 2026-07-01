import { resolve } from 'node:path';
import { loadConfig } from './config/index.js';
import { LLMClient } from './llm/index.js';
import { SkillRegistry } from './skills/index.js';
import { Orchestrator } from './orchestrator/index.js';
import { ContextManager } from './orchestrator/context-manager.js';
import { SessionManager } from './session/index.js';
import { showSplash, parseAtTags, buildUserMessage, createPrompt, prompt, streamOutput, printResponse, printError } from './tui/index.js';

async function main() {
  const config = loadConfig();
  const llm = new LLMClient(config.env.baseUrl, config.env.authToken);

  const skillsDir = resolve(config.appRoot, 'skills');
  const registry = new SkillRegistry();
  registry.loadAll(skillsDir);

  const sessionManager = new SessionManager(config.appRoot, llm);
  const session = sessionManager.create();

  const orchestrator = new Orchestrator({
    llmClient: llm,
    skillRegistry: registry,
    corePrompt: config.corePrompt,
    contextManager: session.context.contextManager,
  });

  showSplash();

  const rl = createPrompt();
  const cwd = process.cwd();

  const loop = async () => {
    const input = await prompt(rl, '\n> ');

    if (!input.trim()) {
      loop();
      return;
    }

    if (input.trim() === '/quit' || input.trim() === '/exit') {
      rl.close();
      process.exit(0);
    }

    const parsed = parseAtTags(input, cwd);
    const userMessage = buildUserMessage(parsed);

    try {
      const response = await orchestrator.run(userMessage);
      printResponse(response);
    } catch (err) {
      printError((err as Error).message);
    }

    loop();
  };

  loop();
}

main().catch((err) => {
  console.error('Fatal:', err.message);
  process.exit(1);
});
