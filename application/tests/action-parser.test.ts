import { describe, it, expect } from 'vitest';
import { parseActions } from '../src/orchestrator/action-parser.js';

describe('parseActions', () => {
  it('parses single [TOOL:name] with JSON params', () => {
    const result = parseActions('[TOOL:web-search] {"query": "test"}');
    expect(result).toEqual([{
      type: 'TOOL',
      target: 'web-search',
      params: { query: 'test' },
    }]);
  });

  it('parses multiple [SUBAGENT:role] lines', () => {
    const input = `[SUBAGENT:researcher] {"task": "Find info", "model": "haiku"}
[SUBAGENT:coder] {"task": "Write code", "model": "sonnet"}`;
    const result = parseActions(input);
    expect(result).toHaveLength(2);
    expect(result[0]).toEqual({
      type: 'SUBAGENT',
      target: 'researcher',
      params: { task: 'Find info', model: 'haiku' },
    });
    expect(result[1]).toEqual({
      type: 'SUBAGENT',
      target: 'coder',
      params: { task: 'Write code', model: 'sonnet' },
    });
  });

  it('parses [RESPOND] with multi-line content', () => {
    const input = `[RESPOND] Here is the answer:
- Point 1
- Point 2
- Point 3`;
    const result = parseActions(input);
    expect(result).toEqual([{
      type: 'RESPOND',
      content: 'Here is the answer:\n- Point 1\n- Point 2\n- Point 3',
    }]);
  });

  it('parses [THINK] content', () => {
    const result = parseActions('[THINK] I need to search for this information');
    expect(result).toEqual([{
      type: 'THINK',
      content: 'I need to search for this information',
    }]);
  });

  it('parses mixed multi-line input in order', () => {
    const input = `[THINK] Let me search first
[TOOL:web-search] {"query": "Node.js LTS"}
[RESPOND] Node.js 22 is the current LTS.`;
    const result = parseActions(input);
    expect(result).toHaveLength(3);
    expect(result[0].type).toBe('THINK');
    expect(result[1].type).toBe('TOOL');
    expect(result[2].type).toBe('RESPOND');
  });

  it('handles malformed JSON gracefully', () => {
    const result = parseActions('[TOOL:web-search] {invalid json}');
    expect(result).toEqual([{
      type: 'TOOL',
      target: 'web-search',
      params: { raw: '{invalid json}' },
    }]);
  });

  it('returns empty array for empty input', () => {
    expect(parseActions('')).toEqual([]);
  });

  it('ignores lines without valid prefixes', () => {
    const input = `Some random text
[RESPOND] Actual response`;
    const result = parseActions(input);
    expect(result).toHaveLength(1);
    expect(result[0].type).toBe('RESPOND');
  });
});
