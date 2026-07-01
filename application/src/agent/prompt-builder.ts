import type { SubAgentSpec } from './index.js';
import type { SkillRegistry } from '../skills/index.js';

export function buildSubAgentPrompt(
  spec: SubAgentSpec,
  skills: SkillRegistry,
  contextSummary: string,
): { systemPrompt: string; userPrompt: string } {
  const toolsSection = skills.getToolsPromptSection();

  const systemPrompt = `# Sub-Agent Instructions

## Role: ${spec.role}

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

## Available Tools
${toolsSection}`;

  const userPrompt = contextSummary
    ? `Context: ${contextSummary}\n\nTask: ${spec.task}`
    : `Task: ${spec.task}`;

  return { systemPrompt, userPrompt };
}
