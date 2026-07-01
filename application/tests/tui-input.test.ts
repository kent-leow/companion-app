import { describe, it, expect } from 'vitest';
import { resolve } from 'node:path';
import { parseAtTags, buildUserMessage } from '../src/tui/input.js';

const FIXTURES = resolve(import.meta.dirname, 'fixtures');

describe('parseAtTags', () => {
  it('extracts @-tagged file paths from input', () => {
    const result = parseAtTags('Check @skills/test-skill/skill.md for issues', FIXTURES);
    expect(result.filePaths).toEqual(['skills/test-skill/skill.md']);
    expect(result.text).toBe('Check  for issues');
  });

  it('reads file contents for valid paths', () => {
    const result = parseAtTags('Look at @skills/test-skill/skill.md', FIXTURES);
    expect(result.fileContents.get('skills/test-skill/skill.md')).toContain('test-skill');
  });

  it('ignores non-existent paths', () => {
    const result = parseAtTags('Check @nonexistent.ts', FIXTURES);
    expect(result.filePaths).toEqual([]);
    expect(result.fileContents.size).toBe(0);
  });

  it('handles multiple @-tags', () => {
    const result = parseAtTags(
      'Compare @skills/test-skill/skill.md and @skills/another-skill/skill.md',
      FIXTURES,
    );
    expect(result.filePaths).toHaveLength(2);
  });
});

describe('buildUserMessage', () => {
  it('appends file contents to message', () => {
    const parsed = parseAtTags('Explain @skills/test-skill/skill.md', FIXTURES);
    const msg = buildUserMessage(parsed);
    expect(msg).toContain('--- Referenced Files ---');
    expect(msg).toContain('### skills/test-skill/skill.md');
    expect(msg).toContain('test-skill');
  });

  it('returns plain text when no files', () => {
    const parsed = parseAtTags('Hello world', FIXTURES);
    const msg = buildUserMessage(parsed);
    expect(msg).toBe('Hello world');
  });
});
