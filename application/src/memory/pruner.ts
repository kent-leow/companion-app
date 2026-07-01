const CHARS_PER_TOKEN = 4;
const MAX_TOKENS = 8000;
const MAX_CHARS = MAX_TOKENS * CHARS_PER_TOKEN;

export function estimateTokens(text: string): number {
  return Math.ceil(text.length / CHARS_PER_TOKEN);
}

export function pruneMemory(content: string): string {
  if (content.length <= MAX_CHARS) return content;

  const sections = content.split(/\n(?=## \[)/);
  while (sections.length > 1 && sections.join('\n').length > MAX_CHARS) {
    sections.shift();
  }

  let result = sections.join('\n');
  if (result.length > MAX_CHARS) {
    result = result.slice(-MAX_CHARS);
  }

  return result;
}
