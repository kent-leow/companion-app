import { describe, it, expect } from 'vitest';
import { resolve } from 'node:path';
import { SkillRegistry } from '../src/skills/index.js';
import { interpolateTemplate, executeSkill } from '../src/skills/executor.js';

const SKILLS_DIR = resolve(import.meta.dirname, '../skills');

describe('Default Skills', () => {
  let registry: SkillRegistry;

  beforeEach(() => {
    registry = new SkillRegistry();
    registry.loadAll(SKILLS_DIR);
  });

  it('loads all 5 default skills without errors', () => {
    expect(registry.get('web-search')).toBeDefined();
    expect(registry.get('web-fetch')).toBeDefined();
    expect(registry.get('read-file')).toBeDefined();
    expect(registry.get('run-command')).toBeDefined();
    expect(registry.get('summarize')).toBeDefined();
    expect(registry.skills.size).toBeGreaterThanOrEqual(5);
  });

  it('web-search has correct triggers', () => {
    const skill = registry.get('web-search')!;
    expect(skill.triggers).toContain('search');
    expect(skill.triggers).toContain('latest');
    expect(skill.triggers).toContain('how to');
  });

  it('web-search command template produces valid curl command', () => {
    const skill = registry.get('web-search')!;
    const cmd = interpolateTemplate(skill.commands[0].template, { query: 'test query' });
    expect(cmd).toContain('curl');
    expect(cmd).toContain('duckduckgo');
    expect(cmd).toContain('test query');
  });

  it('read-file skill reads an actual file', async () => {
    const skill = registry.get('read-file')!;
    const testPath = resolve(import.meta.dirname, '../package.json');
    const result = await executeSkill(skill, { path: testPath });
    expect(result.exitCode).toBe(0);
    expect(result.stdout).toContain('"name": "companion"');
  });

  it('run-command skill executes echo', async () => {
    const skill = registry.get('run-command')!;
    const result = await executeSkill(skill, { command: 'echo hello', cwd: '.' });
    expect(result.exitCode).toBe(0);
    expect(result.stdout).toBe('hello');
  });

  it('summarize skill has no commands (LLM-only)', () => {
    const skill = registry.get('summarize')!;
    expect(skill.commands).toHaveLength(0);
  });
});
