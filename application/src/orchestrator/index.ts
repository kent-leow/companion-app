import { LLMClient, type ChatMessage } from '../llm/index.js';
import { SkillRegistry } from '../skills/index.js';
import { executeSkill } from '../skills/executor.js';
import { parseActions, type ParsedAction } from './action-parser.js';
import { ContextManager } from './context-manager.js';

const MAX_ITERATIONS = 10;
const MAX_RETRIES = 2;

export interface OrchestratorConfig {
  llmClient: LLMClient;
  skillRegistry: SkillRegistry;
  corePrompt: string;
  contextManager: ContextManager;
}

export class Orchestrator {
  private llm: LLMClient;
  private skills: SkillRegistry;
  private corePrompt: string;
  private contextManager: ContextManager;

  constructor(config: OrchestratorConfig) {
    this.llm = config.llmClient;
    this.skills = config.skillRegistry;
    this.corePrompt = config.corePrompt;
    this.contextManager = config.contextManager;
  }

  async run(userMessage: string): Promise<string> {
    const systemPrompt = this.buildSystemPrompt();
    const contextSummary = this.contextManager.getContextForPrompt();

    const userContent = contextSummary
      ? `${contextSummary}\n\n---\nCurrent request: ${userMessage}`
      : userMessage;

    const messages: ChatMessage[] = [
      { role: 'system', content: systemPrompt },
      { role: 'user', content: userContent },
    ];

    let iterations = 0;
    let retries = 0;

    while (iterations < MAX_ITERATIONS) {
      iterations++;

      const response = await this.llm.chat(messages);
      const actions = parseActions(response.content);

      if (actions.length === 0) {
        if (retries < MAX_RETRIES) {
          retries++;
          messages.push({ role: 'assistant', content: response.content });
          messages.push({ role: 'user', content: '[SYSTEM] Your response must start with an action prefix: [RESPOND], [TOOL:<name>], [SUBAGENT:<role>], or [THINK]. Please try again.' });
          continue;
        }
        return response.content;
      }

      retries = 0;

      for (const action of actions) {
        switch (action.type) {
          case 'RESPOND': {
            const result = action.content ?? '';
            this.contextManager.addTurn(userMessage, result);
            await this.contextManager.updateSummary();
            return result;
          }

          case 'TOOL': {
            const toolResult = await this.executeTool(action);
            messages.push({ role: 'assistant', content: response.content });
            messages.push({ role: 'user', content: toolResult });
            break;
          }

          case 'SUBAGENT': {
            const subagentSpecs = actions.filter(a => a.type === 'SUBAGENT');
            const results = await this.runSubAgents(subagentSpecs);
            messages.push({ role: 'assistant', content: response.content });
            messages.push({ role: 'user', content: results });
            break;
          }

          case 'THINK':
            break;
        }

        if (action.type === 'TOOL' || action.type === 'SUBAGENT') break;
      }
    }

    return '[ERROR] Max iterations reached without response';
  }

  private buildSystemPrompt(): string {
    const toolsSection = this.skills.getToolsPromptSection();
    return this.corePrompt.replace('{{TOOLS_SECTION}}', toolsSection);
  }

  private async executeTool(action: ParsedAction): Promise<string> {
    const skill = this.skills.get(action.target!);
    if (!skill) {
      return `[TOOL_ERROR] Unknown tool: ${action.target}`;
    }

    const params = action.params ?? {};
    const result = await executeSkill(skill, params);

    if (result.exitCode !== 0 && !result.timedOut) {
      return `[TOOL_ERROR:${action.target}] Exit ${result.exitCode}: ${result.stderr}`;
    }
    if (result.timedOut) {
      return `[TOOL_ERROR:${action.target}] Timed out after ${skill.commands[0]?.timeout ?? 30}s`;
    }

    return `[TOOL_RESULT:${action.target}]\n${result.stdout}`;
  }

  private async runSubAgents(specs: ParsedAction[]): Promise<string> {
    const results = await Promise.all(
      specs.map(async (spec) => {
        const task = (spec.params as any)?.task ?? 'No task specified';
        const role = spec.target ?? 'general';
        return `[SUBAGENT_RESULT:${role}]\n${task} — (sub-agent execution placeholder)`;
      })
    );
    return results.join('\n\n');
  }
}

export { parseActions } from './action-parser.js';
export { ContextManager } from './context-manager.js';
