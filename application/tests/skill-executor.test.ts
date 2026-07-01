import { describe, it, expect } from 'vitest';
import { interpolateTemplate, executeSkill } from '../src/skills/executor.js';
import type { SkillDef } from '../src/skills/schema.js';

describe('interpolateTemplate', () => {
  it('replaces {param} placeholders with values', () => {
    const result = interpolateTemplate('echo "{input}" | head -{count}', { input: 'hello', count: 5 });
    expect(result).toBe('echo "hello" | head -5');
  });

  it('leaves unmatched placeholders unchanged', () => {
    const result = interpolateTemplate('echo {missing}', {});
    expect(result).toBe('echo {missing}');
  });
});

describe('executeSkill', () => {
  const testSkill: SkillDef = {
    name: 'test',
    version: '1.0.0',
    description: 'Test',
    triggers: [],
    parameters: [],
    commands: [{
      name: 'run',
      template: 'echo "hello {name}"',
      timeout: 5,
    }],
    prompt: '',
  };

  it('executes command with interpolated params', async () => {
    const result = await executeSkill(testSkill, { name: 'world' });
    expect(result.stdout).toBe('hello world');
    expect(result.exitCode).toBe(0);
    expect(result.timedOut).toBe(false);
  });

  it('returns error for unknown command name', async () => {
    const result = await executeSkill(testSkill, {}, 'nonexistent');
    expect(result.exitCode).toBe(1);
    expect(result.stderr).toContain('No command found');
  });

  it('kills process on timeout', async () => {
    const slowSkill: SkillDef = {
      ...testSkill,
      commands: [{ name: 'slow', template: 'sleep 10', timeout: 1 }],
    };
    const result = await executeSkill(slowSkill, {});
    expect(result.timedOut).toBe(true);
  }, 5000);
});
