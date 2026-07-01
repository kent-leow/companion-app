export interface SkillParameter {
  name: string;
  type: 'string' | 'number' | 'boolean' | 'url';
  required: boolean;
  description: string;
}

export interface SkillAuth {
  env: string;
  description: string;
}

export interface SkillCommand {
  name: string;
  template: string;
  timeout: number;
}

export interface SkillDef {
  name: string;
  version: string;
  description: string;
  triggers: string[];
  parameters: SkillParameter[];
  auth?: SkillAuth[];
  commands: SkillCommand[];
  prompt: string;
}
