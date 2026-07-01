import { describe, it, expect } from 'vitest';
import { resolve } from 'node:path';
import { loadSkill } from '../src/skills/loader.js';

const FIXTURES = resolve(import.meta.dirname, 'fixtures/skills');

describe('loadSkill', () => {
  it('parses valid skill.md into correct SkillDef shape', () => {
    const skill = loadSkill(resolve(FIXTURES, 'test-skill/skill.md'));
    expect(skill.name).toBe('test-skill');
    expect(skill.version).toBe('1.0.0');
    expect(skill.description).toBe('A test skill for unit testing');
    expect(skill.triggers).toEqual(['test', 'check', 'verify']);
    expect(skill.parameters).toHaveLength(2);
    expect(skill.parameters[0]).toEqual({
      name: 'input',
      type: 'string',
      required: true,
      description: 'Test input value',
    });
    expect(skill.commands).toHaveLength(1);
    expect(skill.commands[0].name).toBe('run');
    expect(skill.commands[0].timeout).toBe(5);
    expect(skill.prompt).toContain('test skill used in unit tests');
  });

  it('throws on file without YAML frontmatter', () => {
    const badPath = resolve(import.meta.dirname, '../vitest.config.ts');
    expect(() => loadSkill(badPath)).toThrow('Invalid skill.md format');
  });
});
