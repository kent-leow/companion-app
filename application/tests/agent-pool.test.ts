import { describe, it, expect, vi } from 'vitest';
import { LLMClient } from '../src/llm/client.js';
import { SkillRegistry } from '../src/skills/index.js';
import { runSubAgentsParallel, formatPoolResults } from '../src/agent/pool.js';
import { runSubAgent } from '../src/agent/index.js';

describe('runSubAgentsParallel', () => {
  it('runs agents concurrently and collects results', async () => {
    const llm = new LLMClient('https://fake.api', 'key');
    vi.spyOn(llm, 'chat').mockResolvedValue({
      content: '[RESPOND] Done',
      model: 'sonnet',
      usage: undefined,
    });

    const registry = new SkillRegistry();
    const results = await runSubAgentsParallel(
      [
        { role: 'researcher', task: 'Find info' },
        { role: 'coder', task: 'Write code' },
      ],
      llm,
      registry,
      'context',
    );

    expect(results).toHaveLength(2);
    expect(results[0].role).toBe('researcher');
    expect(results[0].output).toBe('Done');
    expect(results[1].role).toBe('coder');
    expect(results[1].output).toBe('Done');
  });

  it('handles timeout gracefully', async () => {
    const llm = new LLMClient('https://fake.api', 'key');
    vi.spyOn(llm, 'chat').mockImplementation(
      () => new Promise((resolve) => setTimeout(() => resolve({ content: '[RESPOND] late', model: 'x', usage: undefined }), 5000))
    );

    const registry = new SkillRegistry();
    const results = await runSubAgentsParallel(
      [{ role: 'slow', task: 'Take forever' }],
      llm,
      registry,
      '',
      100,
    );

    expect(results[0].error).toBe('timeout');
  });
});

describe('runSubAgent', () => {
  it('cannot spawn nested sub-agents', async () => {
    const llm = new LLMClient('https://fake.api', 'key');
    let callCount = 0;
    vi.spyOn(llm, 'chat').mockImplementation(async () => {
      callCount++;
      if (callCount === 1) {
        return { content: '[SUBAGENT:nested] {"task": "illegal"}', model: 'x', usage: undefined };
      }
      return { content: '[RESPOND] Got the rejection message', model: 'x', usage: undefined };
    });

    const registry = new SkillRegistry();
    const result = await runSubAgent(
      { role: 'test', task: 'Try nesting' },
      llm,
      registry,
      '',
    );

    expect(result).toBe('Got the rejection message');
    expect(callCount).toBe(2);
  });
});

describe('formatPoolResults', () => {
  it('formats results with role headers', () => {
    const formatted = formatPoolResults([
      { role: 'researcher', output: 'Found 3 results' },
      { role: 'coder', output: 'Function written' },
    ]);
    expect(formatted).toContain('[SUBAGENT_RESULT:researcher]');
    expect(formatted).toContain('Found 3 results');
    expect(formatted).toContain('[SUBAGENT_RESULT:coder]');
  });

  it('formats errors', () => {
    const formatted = formatPoolResults([
      { role: 'broken', output: '', error: 'timeout' },
    ]);
    expect(formatted).toContain('ERROR: timeout');
  });
});
