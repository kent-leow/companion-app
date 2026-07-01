import { describe, it, expect } from 'vitest';
import { resolve } from 'node:path';
import { buildSubAgentPrompt } from '../src/agent/prompt-builder.js';
import { SkillRegistry } from '../src/skills/index.js';

const FIXTURES = resolve(import.meta.dirname, 'fixtures/skills');

describe('buildSubAgentPrompt', () => {
  it('produces system prompt with role and tools', () => {
    const registry = new SkillRegistry();
    registry.loadAll(FIXTURES);

    const { systemPrompt, userPrompt } = buildSubAgentPrompt(
      { role: 'researcher', task: 'Find info about Node.js' },
      registry,
      'User asked about JavaScript runtimes',
    );

    expect(systemPrompt).toContain('## Role: researcher');
    expect(systemPrompt).toContain('You CANNOT spawn sub-agents');
    expect(systemPrompt).toContain('Available tools');
    expect(systemPrompt).toContain('test-skill');
    expect(userPrompt).toContain('Context: User asked about JavaScript runtimes');
    expect(userPrompt).toContain('Task: Find info about Node.js');
  });

  it('omits context when empty', () => {
    const registry = new SkillRegistry();
    const { userPrompt } = buildSubAgentPrompt(
      { role: 'coder', task: 'Write a function' },
      registry,
      '',
    );
    expect(userPrompt).toBe('Task: Write a function');
    expect(userPrompt).not.toContain('Context:');
  });
});
