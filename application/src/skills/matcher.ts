import type { SkillRegistry } from './index.js';
import type { SkillDef } from './schema.js';

export function matchSkill(query: string, registry: SkillRegistry): SkillDef | null {
  const lower = query.toLowerCase();

  for (const [trigger, skillName] of registry.triggerMap) {
    if (lower.includes(trigger)) {
      return registry.get(skillName) ?? null;
    }
  }

  for (const [, skill] of registry.skills) {
    const descWords = skill.description.toLowerCase().split(/\s+/);
    const queryWords = lower.split(/\s+/);
    const overlap = queryWords.filter(w => descWords.includes(w) && w.length > 3);
    if (overlap.length >= 2) {
      return skill;
    }
  }

  return null;
}
