import { describe, it, expect } from 'vitest';
import { pruneMemory, estimateTokens } from '../src/memory/pruner.js';

describe('estimateTokens', () => {
  it('estimates roughly 1 token per 4 chars', () => {
    expect(estimateTokens('hello world')).toBe(3); // 11 chars / 4 = 2.75 → 3
  });
});

describe('pruneMemory', () => {
  it('returns content unchanged if under budget', () => {
    const short = '## [2024-01-01T00:00]\nShort entry';
    expect(pruneMemory(short)).toBe(short);
  });

  it('removes oldest entries when over 8K tokens', () => {
    const entries: string[] = [];
    for (let i = 0; i < 100; i++) {
      entries.push(`## [2024-01-${String(i + 1).padStart(2, '0')}T00:00]\n${'x'.repeat(500)}`);
    }
    const content = entries.join('\n');
    expect(content.length).toBeGreaterThan(32000);

    const pruned = pruneMemory(content);
    expect(pruned.length).toBeLessThanOrEqual(32000);
    expect(pruned).not.toContain('2024-01-01T00:00');
    expect(pruned).toContain('2024-01-100T00:00');
  });
});
