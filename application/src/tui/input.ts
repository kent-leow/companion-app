import { createInterface, type Interface } from 'node:readline';
import { readFileSync, existsSync } from 'node:fs';
import { resolve } from 'node:path';

export interface ParsedInput {
  text: string;
  filePaths: string[];
  fileContents: Map<string, string>;
}

export function parseAtTags(input: string, cwd: string): ParsedInput {
  const filePaths: string[] = [];
  const fileContents = new Map<string, string>();

  const tagPattern = /@([\w.\/\-]+)/g;
  let text = input;
  let match: RegExpExecArray | null;

  while ((match = tagPattern.exec(input)) !== null) {
    const rawPath = match[1];
    const fullPath = resolve(cwd, rawPath);

    if (existsSync(fullPath)) {
      filePaths.push(rawPath);
      try {
        const content = readFileSync(fullPath, 'utf-8');
        const truncated = content.length > 10000 ? content.slice(0, 10000) + '\n...[truncated]' : content;
        fileContents.set(rawPath, truncated);
      } catch {
        fileContents.set(rawPath, '[ERROR: could not read file]');
      }
    }
  }

  text = text.replace(tagPattern, '').trim();

  return { text, filePaths, fileContents };
}

export function buildUserMessage(parsed: ParsedInput): string {
  let message = parsed.text;

  if (parsed.fileContents.size > 0) {
    message += '\n\n--- Referenced Files ---';
    for (const [path, content] of parsed.fileContents) {
      message += `\n\n### ${path}\n\`\`\`\n${content}\n\`\`\``;
    }
  }

  return message;
}

export function createPrompt(rl?: Interface): Interface {
  return rl ?? createInterface({
    input: process.stdin,
    output: process.stdout,
  });
}

export function prompt(rl: Interface, question: string): Promise<string> {
  return new Promise((resolve) => {
    rl.question(question, (answer) => {
      resolve(answer);
    });
  });
}
