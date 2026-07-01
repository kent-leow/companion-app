import { describe, it, expect } from 'vitest';
import { selectModel } from '../src/llm/model-selector.js';

describe('selectModel', () => {
  it('returns haiku for simple greetings', () => {
    expect(selectModel([{ role: 'user', content: 'hello' }])).toBe('bedrock.claude-haiku-4-5');
    expect(selectModel([{ role: 'user', content: 'hi' }])).toBe('bedrock.claude-haiku-4-5');
    expect(selectModel([{ role: 'user', content: 'thanks' }])).toBe('bedrock.claude-haiku-4-5');
  });

  it('returns sonnet for standard queries', () => {
    expect(selectModel([{ role: 'user', content: 'How do I use async/await in TypeScript?' }]))
      .toBe('bedrock.claude-sonnet-4-6');
  });

  it('returns opus for complex reasoning tasks', () => {
    expect(selectModel([{ role: 'user', content: 'Analyze the trade-offs between microservices and monolith architecture for our e-commerce platform' }]))
      .toBe('bedrock.claude-opus-4-6');
  });

  it('returns opus for long inputs (>200 chars)', () => {
    const longInput = 'a'.repeat(201);
    expect(selectModel([{ role: 'user', content: longInput }]))
      .toBe('bedrock.claude-opus-4-6');
  });

  it('returns sonnet as default when no messages', () => {
    expect(selectModel([])).toBe('bedrock.claude-sonnet-4-6');
  });

  it('handles multimodal content parts', () => {
    expect(selectModel([{
      role: 'user',
      content: [{ type: 'text', text: 'hello' }],
    }])).toBe('bedrock.claude-haiku-4-5');
  });
});
