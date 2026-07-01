import { readFileSync, existsSync } from 'node:fs';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { loadEnv, type EnvConfig } from './env.js';

const __dirname = dirname(fileURLToPath(import.meta.url));

function findAppRoot(): string {
  let candidate = resolve(__dirname, '../..');
  if (existsSync(resolve(candidate, 'core.md'))) return candidate;
  candidate = resolve(__dirname, '..');
  if (existsSync(resolve(candidate, 'core.md'))) return candidate;
  return resolve(__dirname, '../..');
}

const APP_ROOT = findAppRoot();

export interface AppConfig {
  env: EnvConfig;
  corePrompt: string;
  appRoot: string;
}

export function loadConfig(): AppConfig {
  const env = loadEnv();
  const corePromptPath = resolve(APP_ROOT, 'core.md');
  const corePrompt = readFileSync(corePromptPath, 'utf-8');

  return { env, corePrompt, appRoot: APP_ROOT };
}

export { loadEnv, type EnvConfig };
