import { describe, it, expect } from 'vitest';
import { resolve } from 'node:path';
import { SkillRegistry } from '../src/skills/index.js';

const FIXTURES = resolve(import.meta.dirname, 'fixtures/skills');

describe('SkillRegistry', () => {
  it('loads all skills from directory', () => {
    const registry = new SkillRegistry();
    registry.loadAll(FIXTURES);
    expect(registry.skills.size).toBe(2);
    expect(registry.get('test-skill')).toBeDefined();
    expect(registry.get('another-skill')).toBeDefined();
  });

  it('builds triggerMap from multiple skills', () => {
    const registry = new SkillRegistry();
    registry.loadAll(FIXTURES);
    expect(registry.triggerMap.get('test')).toBe('test-skill');
    expect(registry.triggerMap.get('check')).toBe('test-skill');
    expect(registry.triggerMap.get('search')).toBe('another-skill');
    expect(registry.triggerMap.get('find')).toBe('another-skill');
  });

  it('generates Available Tools prompt section', () => {
    const registry = new SkillRegistry();
    registry.loadAll(FIXTURES);
    const section = registry.getToolsPromptSection();
    expect(section).toContain('Available tools:');
    expect(section).toContain('test-skill: A test skill for unit testing');
    expect(section).toContain('another-skill: Another test skill');
    expect(section).toContain('[TOOL:<name>]');
  });

  it('returns empty message when no skills loaded', () => {
    const registry = new SkillRegistry();
    expect(registry.getToolsPromptSection()).toBe('No tools available.');
  });

  it('handles non-existent directory gracefully', () => {
    const registry = new SkillRegistry();
    registry.loadAll('/nonexistent/path');
    expect(registry.skills.size).toBe(0);
  });
});
