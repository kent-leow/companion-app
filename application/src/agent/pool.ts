import { LLMClient } from '../llm/index.js';
import { SkillRegistry } from '../skills/index.js';
import { runSubAgent, type SubAgentSpec } from './index.js';

const DEFAULT_TIMEOUT = 60000;

export interface PoolResult {
  role: string;
  output: string;
  error?: string;
}

export async function runSubAgentsParallel(
  specs: SubAgentSpec[],
  llm: LLMClient,
  skills: SkillRegistry,
  contextSummary: string,
  timeout: number = DEFAULT_TIMEOUT,
): Promise<PoolResult[]> {
  const tasks = specs.map(async (spec): Promise<PoolResult> => {
    try {
      const output = await Promise.race([
        runSubAgent(spec, llm, skills, contextSummary),
        new Promise<string>((_, reject) =>
          setTimeout(() => reject(new Error('timeout')), timeout)
        ),
      ]);
      return { role: spec.role, output };
    } catch (err) {
      return { role: spec.role, output: '', error: (err as Error).message };
    }
  });

  return Promise.all(tasks);
}

export function formatPoolResults(results: PoolResult[]): string {
  return results.map(r => {
    if (r.error) {
      return `[SUBAGENT_RESULT:${r.role}] ERROR: ${r.error}`;
    }
    return `[SUBAGENT_RESULT:${r.role}]\n${r.output}`;
  }).join('\n\n');
}
