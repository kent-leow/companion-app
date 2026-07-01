import { describe, it, expect } from 'vitest';
import { resolve } from 'node:path';
import { SkillRegistry } from '../src/skills/index.js';
import { matchSkill } from '../src/skills/matcher.js';

const FIXTURES = resolve(import.meta.dirname, 'fixtures/skills');

describe('matchSkill', () => {
  let registry: SkillRegistry;

  beforeEach(() => {
    registry = new SkillRegistry();
    registry.loadAll(FIXTURES);
  });

  it('returns correct skill for trigger keywords', () => {
    const skill = matchSkill('test this function', registry);
    expect(skill?.name).toBe('test-skill');
  });

  it('matches search trigger to another-skill', () => {
    const skill = matchSkill('search the web for Node.js', registry);
    expect(skill?.name).toBe('another-skill');
  });

  it('returns null for unmatched queries', () => {
    const skill = matchSkill('completely unrelated banana topic', registry);
    expect(skill).toBeNull();
  });

  it('is case-insensitive', () => {
    const skill = matchSkill('FIND something', registry);
    expect(skill?.name).toBe('another-skill');
  });
});
