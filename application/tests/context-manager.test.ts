import { describe, it, expect, beforeEach } from 'vitest';
import { ContextManager } from '../src/orchestrator/context-manager.js';

describe('ContextManager', () => {
  let ctx: ContextManager;

  beforeEach(() => {
    ctx = new ContextManager();
  });

  it('returns empty string when no turns', () => {
    expect(ctx.getContextForPrompt()).toBe('');
  });

  it('returns raw messages for first 3 turns', () => {
    ctx.addTurn('Hello', 'Hi there');
    ctx.addTurn('How are you?', 'Good');
    ctx.addTurn('What is Node?', 'A JS runtime');

    const context = ctx.getContextForPrompt();
    expect(context).toContain('Turn 1:');
    expect(context).toContain('Hello');
    expect(context).toContain('Hi there');
    expect(context).toContain('Turn 3:');
  });

  it('triggers summarization after 4+ turns', async () => {
    ctx.addTurn('Q1', 'A1');
    ctx.addTurn('Q2', 'A2');
    ctx.addTurn('Q3', 'A3');
    ctx.addTurn('Q4', 'A4');

    await ctx.updateSummary();
    const context = ctx.getContextForPrompt();
    expect(context).toContain('Prior conversation summary');
    expect(context).toContain('Q3');
    expect(context).toContain('Q4');
  });

  it('caps context at 8000 chars', () => {
    const longMsg = 'x'.repeat(5000);
    ctx.addTurn(longMsg, longMsg);
    ctx.addTurn(longMsg, longMsg);
    ctx.addTurn(longMsg, longMsg);
    ctx.addTurn(longMsg, longMsg);

    const context = ctx.getContextForPrompt();
    expect(context.length).toBeLessThanOrEqual(8003); // 8000 + '...'
  });

  it('getSubAgentContext returns short summary', () => {
    ctx.addTurn('How do I deploy to AWS?', 'Use ECS or Lambda');
    const subCtx = ctx.getSubAgentContext();
    expect(subCtx).toContain('deploy to AWS');
    expect(subCtx.length).toBeLessThan(500);
  });

  it('getState returns correct structure', () => {
    ctx.addTurn('Hello', 'Hi');
    const state = ctx.getState();
    expect(state.turnCount).toBe(1);
    expect(state.lastUserMessage).toBe('Hello');
    expect(state.lastAssistantResponse).toBe('Hi');
  });
});
