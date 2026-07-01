import { describe, it, expect, beforeEach } from 'vitest';
import { resolve } from 'node:path';
import { SkillRegistry } from '../src/skills/index.js';
import { executeSkill } from '../src/skills/executor.js';

const SKILLS_DIR = resolve(import.meta.dirname, '../skills');

describe('Auth-Required Skills', () => {
  let registry: SkillRegistry;

  beforeEach(() => {
    registry = new SkillRegistry();
    registry.loadAll(SKILLS_DIR);
  });

  it('loads all 6 auth skills', () => {
    expect(registry.get('figma-design-context')).toBeDefined();
    expect(registry.get('fix-vulnerabilities')).toBeDefined();
    expect(registry.get('git-apis')).toBeDefined();
    expect(registry.get('git-workflow')).toBeDefined();
    expect(registry.get('gitlab-mr-automation')).toBeDefined();
    expect(registry.get('jira-ticket')).toBeDefined();
  });

  it('total skills loaded is 11 (5 default + 6 auth)', () => {
    expect(registry.skills.size).toBe(11);
  });

  it('auth skills have auth field defined', () => {
    const figma = registry.get('figma-design-context')!;
    expect(figma.auth).toBeDefined();
    expect(figma.auth!.some(a => a.env === 'FIGMA_TOKEN')).toBe(true);

    const jira = registry.get('jira-ticket')!;
    expect(jira.auth!.some(a => a.env === 'JIRA_TOKEN')).toBe(true);
  });

  it('preflight commands detect missing env vars', async () => {
    const gitApis = registry.get('git-apis')!;
    const result = await executeSkill(gitApis, {}, 'preflight');
    expect(result.stdout).toContain('GitLab:');
    expect(result.stdout).toContain('GitHub:');
  });

  it('figma preflight reports MISSING without token', async () => {
    const figma = registry.get('figma-design-context')!;
    const result = await executeSkill(figma, {}, 'preflight');
    expect(result.stdout).toContain('FIGMA_TOKEN:');
  });
});
