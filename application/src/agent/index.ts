import { LLMClient, type ChatMessage } from '../llm/index.js';
import { SkillRegistry } from '../skills/index.js';
import { executeSkill } from '../skills/executor.js';
import { parseActions } from '../orchestrator/action-parser.js';
import { buildSubAgentPrompt } from './prompt-builder.js';
import type { ModelId } from '../llm/model-selector.js';

export interface SubAgentSpec {
  role: string;
  task: string;
  modelHint?: string;
  skill?: string;
  contextNeeded?: string;
}

const SUB_AGENT_MAX_ITERATIONS = 5;

export async function runSubAgent(
  spec: SubAgentSpec,
  llm: LLMClient,
  skills: SkillRegistry,
  contextSummary: string,
): Promise<string> {
  const { systemPrompt, userPrompt } = buildSubAgentPrompt(spec, skills, contextSummary);

  const messages: ChatMessage[] = [
    { role: 'system', content: systemPrompt },
    { role: 'user', content: userPrompt },
  ];

  const model = (spec.modelHint as ModelId) ?? 'bedrock.claude-sonnet-4-6';
  let iterations = 0;

  while (iterations < SUB_AGENT_MAX_ITERATIONS) {
    iterations++;
    const response = await llm.chat(messages, { model });
    const actions = parseActions(response.content);

    if (actions.length === 0) return response.content;

    for (const action of actions) {
      switch (action.type) {
        case 'RESPOND':
          return action.content ?? '';

        case 'TOOL': {
          const skill = skills.get(action.target!);
          if (!skill) {
            messages.push({ role: 'assistant', content: response.content });
            messages.push({ role: 'user', content: `[TOOL_ERROR] Unknown tool: ${action.target}` });
            break;
          }
          const result = await executeSkill(skill, action.params ?? {});
          const resultStr = result.exitCode === 0
            ? `[TOOL_RESULT:${action.target}]\n${result.stdout}`
            : `[TOOL_ERROR:${action.target}] ${result.stderr}`;
          messages.push({ role: 'assistant', content: response.content });
          messages.push({ role: 'user', content: resultStr });
          break;
        }

        case 'SUBAGENT':
          messages.push({ role: 'assistant', content: response.content });
          messages.push({ role: 'user', content: '[SYSTEM] Sub-agents cannot spawn nested sub-agents. Use [TOOL] or [RESPOND] only.' });
          break;

        case 'THINK':
          break;
      }

      if (action.type === 'TOOL' || action.type === 'SUBAGENT') break;
    }
  }

  return '[ERROR] Sub-agent max iterations reached';
}
