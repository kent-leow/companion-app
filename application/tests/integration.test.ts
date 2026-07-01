import { describe, it, expect, vi } from 'vitest';
import { resolve } from 'node:path';
import { parseAtTags, buildUserMessage } from '../src/tui/input.js';
import { LLMClient } from '../src/llm/client.js';
import { SkillRegistry } from '../src/skills/index.js';
import { Orchestrator } from '../src/orchestrator/index.js';
import { ContextManager } from '../src/orchestrator/context-manager.js';

const FIXTURES = resolve(import.meta.dirname, 'fixtures');
const SKILLS_DIR = resolve(import.meta.dirname, '../skills');

describe('Integration', () => {
  it('@-tag extracts path and injects file content into message', () => {
    const parsed = parseAtTags('Explain @skills/test-skill/skill.md', FIXTURES);
    const msg = buildUserMessage(parsed);
    expect(msg).toContain('--- Referenced Files ---');
    expect(msg).toContain('test-skill');
    expect(msg).toContain('A test skill for unit testing');
  });

  it('full turn: user input → orchestrator → response', async () => {
    const llm = new LLMClient('https://fake.api', 'key');
    vi.spyOn(llm, 'chat').mockResolvedValue({
      content: '[RESPOND] TypeScript is a typed superset of JavaScript.',
      model: 'sonnet',
      usage: undefined,
    });

    const registry = new SkillRegistry();
    registry.loadAll(SKILLS_DIR);

    const ctx = new ContextManager();
    const orchestrator = new Orchestrator({
      llmClient: llm,
      skillRegistry: registry,
      corePrompt: 'System\n{{TOOLS_SECTION}}',
      contextManager: ctx,
    });

    const response = await orchestrator.run('What is TypeScript?');
    expect(response).toBe('TypeScript is a typed superset of JavaScript.');
    expect(ctx.turnCount).toBe(1);
  });

  it('tool call integration: orchestrator executes skill and responds', async () => {
    const llm = new LLMClient('https://fake.api', 'key');
    let callCount = 0;
    vi.spyOn(llm, 'chat').mockImplementation(async () => {
      callCount++;
      if (callCount === 1) {
        return { content: '[TOOL:read-file] {"path": "' + resolve(import.meta.dirname, '../package.json') + '"}', model: 'sonnet', usage: undefined };
      }
      return { content: '[RESPOND] The package name is "companion"', model: 'sonnet', usage: undefined };
    });

    const registry = new SkillRegistry();
    registry.loadAll(SKILLS_DIR);

    const ctx = new ContextManager();
    const orchestrator = new Orchestrator({
      llmClient: llm,
      skillRegistry: registry,
      corePrompt: 'System\n{{TOOLS_SECTION}}',
      contextManager: ctx,
    });

    const response = await orchestrator.run('Read my package.json');
    expect(response).toBe('The package name is "companion"');
  });
});
