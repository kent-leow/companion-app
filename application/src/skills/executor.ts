import { spawn } from 'node:child_process';
import type { SkillDef, SkillCommand } from './schema.js';

export interface ExecutionResult {
  stdout: string;
  stderr: string;
  exitCode: number;
  timedOut: boolean;
}

export function interpolateTemplate(template: string, params: Record<string, unknown>): string {
  return template.replace(/\{(\w+)\}/g, (match, key) => {
    if (key in params && params[key] !== undefined && params[key] !== '') return String(params[key]);
    return match;
  });
}

export async function executeSkill(
  skill: SkillDef,
  params: Record<string, unknown>,
  commandName?: string,
): Promise<ExecutionResult> {
  const command = commandName
    ? skill.commands.find(c => c.name === commandName)
    : skill.commands[0];

  if (!command) {
    return { stdout: '', stderr: `No command found: ${commandName ?? 'default'}`, exitCode: 1, timedOut: false };
  }

  return executeCommand(command, params);
}

export function executeCommand(command: SkillCommand, params: Record<string, unknown>): Promise<ExecutionResult> {
  const script = interpolateTemplate(command.template, params);
  const timeoutMs = command.timeout * 1000;

  return new Promise((resolve) => {
    let stdout = '';
    let stderr = '';
    let timedOut = false;

    const proc = spawn('bash', ['-c', script], {
      env: process.env,
      stdio: ['ignore', 'pipe', 'pipe'],
    });

    const timer = setTimeout(() => {
      timedOut = true;
      proc.kill('SIGTERM');
      setTimeout(() => proc.kill('SIGKILL'), 1000);
    }, timeoutMs);

    proc.stdout.on('data', (data) => { stdout += data.toString(); });
    proc.stderr.on('data', (data) => { stderr += data.toString(); });

    proc.on('close', (code) => {
      clearTimeout(timer);
      resolve({ stdout: stdout.trim(), stderr: stderr.trim(), exitCode: code ?? 1, timedOut });
    });

    proc.on('error', (err) => {
      clearTimeout(timer);
      resolve({ stdout: '', stderr: err.message, exitCode: 1, timedOut: false });
    });
  });
}
