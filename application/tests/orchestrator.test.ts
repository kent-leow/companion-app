import { describe, it, expect, vi, beforeEach } from 'vitest';
import { Orchestrator } from '../src/orchestrator/index.js';
import { LLMClient } from '../src/llm/client.js';
import { SkillRegistry } from '../src/skills/index.js';
import { ContextManager } from '../src/orchestrator/context-manager.js';

function createMockOrchestrator(responses: string[]) {
  const llm = new LLMClient('https://fake.api', 'fake-key');
  let callIndex = 0;
  vi.spyOn(llm, 'chat').mockImplementation(async () => {
    const content = responses[callIndex] ?? '[RESPOND] fallback';
    callIndex++;
    return { content, model: 'sonnet', usage: undefined };
  });

  const registry = new SkillRegistry();
  registry.skills.set('echo-tool', {
    name: 'echo-tool',
    version: '1.0.0',
    description: 'Echo tool',
    triggers: ['echo'],
    parameters: [{ name: 'text', type: 'string', required: true, description: 'Text to echo' }],
    commands: [{ name: 'run', template: 'echo "{text}"', timeout: 5 }],
    prompt: '',
  });

  const ctx = new ContextManager();
  const orch = new Orchestrator({
    llmClient: llm,
    skillRegistry: registry,
    corePrompt: 'System prompt\n{{TOOLS_SECTION}}',
    contextManager: ctx,
  });

  return { orch, llm, ctx };
}

describe('Orchestrator', () => {
  it('terminates on [RESPOND] and returns content', async () => {
    const { orch } = createMockOrchestrator(['[RESPOND] The answer is 42']);
    const result = await orch.run('What is 6*7?');
    expect(result).toBe('The answer is 42');
  });

  it('executes TOOL action and feeds result back to LLM', async () => {
    const { orch, llm } = createMockOrchestrator([
      '[TOOL:echo-tool] {"text": "hello world"}',
      '[RESPOND] The tool said: hello world',
    ]);

    const result = await orch.run('Echo hello world');
    expect(result).toBe('The tool said: hello world');
    expect(llm.chat).toHaveBeenCalledTimes(2);
  });

  it('returns error for unknown tool', async () => {
    const { orch } = createMockOrchestrator([
      '[TOOL:nonexistent] {"query": "test"}',
      '[RESPOND] Tool not found',
    ]);

    const result = await orch.run('Use unknown tool');
    expect(result).toBe('Tool not found');
  });

  it('stops after MAX_ITERATIONS with error', async () => {
    const neverRespond = Array(15).fill('[THINK] Still thinking...\n[TOOL:echo-tool] {"text": "loop"}');
    const { orch } = createMockOrchestrator(neverRespond);
    const result = await orch.run('Infinite loop test');
    expect(result).toContain('Max iterations reached');
  });

  it('retries on malformed response (no prefix)', async () => {
    const { orch, llm } = createMockOrchestrator([
      'Just some text without a prefix',
      '[RESPOND] Fixed response',
    ]);

    const result = await orch.run('Test retry');
    expect(result).toBe('Fixed response');
    expect(llm.chat).toHaveBeenCalledTimes(2);
  });

  it('updates context manager after successful turn', async () => {
    const { orch, ctx } = createMockOrchestrator(['[RESPOND] Hello!']);
    await orch.run('Hi there');
    expect(ctx.turnCount).toBe(1);
    const state = ctx.getState();
    expect(state.lastUserMessage).toBe('Hi there');
    expect(state.lastAssistantResponse).toBe('Hello!');
  });
});
