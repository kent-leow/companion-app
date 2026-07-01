import { readdirSync, existsSync } from 'node:fs';
import { resolve, join } from 'node:path';
import { loadSkill } from './loader.js';
import type { SkillDef } from './schema.js';

export class SkillRegistry {
  skills: Map<string, SkillDef> = new Map();
  triggerMap: Map<string, string> = new Map();

  loadAll(skillsDir: string): void {
    if (!existsSync(skillsDir)) return;

    const entries = readdirSync(skillsDir, { withFileTypes: true });
    for (const entry of entries) {
      if (!entry.isDirectory()) continue;

      const skillPath = join(skillsDir, entry.name, 'skill.md');
      if (!existsSync(skillPath)) continue;

      try {
        const skill = loadSkill(skillPath);
        this.skills.set(skill.name, skill);

        for (const trigger of skill.triggers) {
          this.triggerMap.set(trigger.toLowerCase(), skill.name);
        }

        if (skill.auth) {
          for (const auth of skill.auth) {
            if (!process.env[auth.env]) {
              console.warn(`[skill:${skill.name}] missing env: ${auth.env}`);
            }
          }
        }
      } catch (err) {
        console.warn(`[skill:${entry.name}] failed to load: ${(err as Error).message}`);
      }
    }
  }

  get(name: string): SkillDef | undefined {
    return this.skills.get(name);
  }

  getToolsPromptSection(): string {
    if (this.skills.size === 0) return 'No tools available.';

    const lines = [
      'You MUST use action prefixes to invoke tools. Output format:',
      '[TOOL:<name>] {"param": "value", ...}',
      '',
      'Available tools:',
    ];

    for (const [, skill] of this.skills) {
      const params = skill.parameters
        .map(p => `"${p.name}": "<${p.description}>"`)
        .join(', ');
      lines.push(`- ${skill.name}: ${skill.description}. Params: {${params}}`);
    }

    lines.push('');
    lines.push('When you need information or action, use [TOOL:name]. After receiving tool output, continue reasoning.');
    lines.push('When ready to answer the user, use [RESPOND].');
    lines.push('When spawning parallel work, use [SUBAGENT:role].');

    return lines.join('\n');
  }
}

export { loadSkill } from './loader.js';
export type { SkillDef } from './schema.js';
