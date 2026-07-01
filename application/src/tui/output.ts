import chalk from 'chalk';

export async function streamOutput(tokens: AsyncIterable<string>): Promise<string> {
  let full = '';
  for await (const token of tokens) {
    process.stdout.write(token);
    full += token;
  }
  process.stdout.write('\n');
  return full;
}

export function printResponse(text: string): void {
  console.log(formatMarkdown(text));
}

export function printError(message: string): void {
  console.error(chalk.red(`✗ ${message}`));
}

export function printInfo(message: string): void {
  console.log(chalk.dim(`  ${message}`));
}

export function formatMarkdown(text: string): string {
  return text
    .replace(/^### (.+)$/gm, chalk.bold.white('$1'))
    .replace(/^## (.+)$/gm, chalk.bold.cyan('$1'))
    .replace(/^# (.+)$/gm, chalk.bold.green('$1'))
    .replace(/`([^`]+)`/g, chalk.yellow('$1'))
    .replace(/\*\*([^*]+)\*\*/g, chalk.bold('$1'))
    .replace(/^- (.+)$/gm, chalk.dim('  • ') + '$1')
    .replace(/^(\d+)\. (.+)$/gm, chalk.dim('  $1. ') + '$2');
}
