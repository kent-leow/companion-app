import { describe, it, expect } from 'vitest';
import { formatMarkdown } from '../src/tui/output.js';

describe('formatMarkdown', () => {
  it('formats inline code', () => {
    const result = formatMarkdown('Use `npm install` to install');
    expect(result).toContain('npm install');
    expect(result).not.toContain('`');
  });

  it('formats bold text', () => {
    const result = formatMarkdown('This is **important**');
    expect(result).toContain('important');
    expect(result).not.toContain('**');
  });

  it('formats bullet lists', () => {
    const result = formatMarkdown('- First item\n- Second item');
    expect(result).toContain('•');
    expect(result).toContain('First item');
  });

  it('formats numbered lists', () => {
    const result = formatMarkdown('1. First\n2. Second');
    expect(result).toContain('1.');
    expect(result).toContain('First');
  });
});
