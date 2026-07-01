import { readFileSync } from 'node:fs';
import { parse as parseYaml } from 'yaml';
import type { SkillDef } from './schema.js';

export function loadSkill(filePath: string): SkillDef {
  const raw = readFileSync(filePath, 'utf-8');
  const fmMatch = raw.match(/^---\n([\s\S]*?)\n---\n([\s\S]*)$/);

  if (!fmMatch) {
    throw new Error(`Invalid skill.md format (no YAML frontmatter): ${filePath}`);
  }

  const frontmatter = parseYaml(fmMatch[1]);
  const body = fmMatch[2].trim();

  if (!frontmatter.name || !frontmatter.description) {
    throw new Error(`Skill missing required fields (name, description): ${filePath}`);
  }

  return {
    name: frontmatter.name,
    version: frontmatter.version ?? '1.0.0',
    description: frontmatter.description,
    triggers: frontmatter.triggers ?? [],
    parameters: (frontmatter.parameters ?? []).map((p: any) => ({
      name: p.name,
      type: p.type ?? 'string',
      required: p.required ?? false,
      description: p.description ?? '',
    })),
    auth: frontmatter.auth,
    commands: (frontmatter.commands ?? []).map((c: any) => ({
      name: c.name,
      template: c.template,
      timeout: c.timeout ?? 30,
    })),
    prompt: body,
  };
}
