import type { ChatMessage } from './client.js';

export type ModelId =
  | 'bedrock.claude-haiku-4-5'
  | 'bedrock.claude-sonnet-4-6'
  | 'bedrock.claude-opus-4-6';

const SIMPLE_PATTERNS = [
  /^(hi|hello|hey|thanks|ok|yes|no|bye)\b/i,
  /^what (is|are) \w+$/i,
  /^(define|translate|convert)\b/i,
];

const COMPLEX_PATTERNS = [
  /\b(analyze|architect|design|implement|refactor|debug|security audit)\b/i,
  /\b(compare and contrast|trade.?offs|pros and cons)\b/i,
  /\b(step by step|detailed plan|comprehensive)\b/i,
];

export function selectModel(messages: ChatMessage[]): ModelId {
  const lastUser = messages.filter(m => m.role === 'user').pop();
  if (!lastUser) return 'bedrock.claude-sonnet-4-6';

  const content = typeof lastUser.content === 'string'
    ? lastUser.content
    : lastUser.content.filter(p => p.type === 'text').map(p => p.text).join(' ');

  if (content.length < 50 && SIMPLE_PATTERNS.some(p => p.test(content))) {
    return 'bedrock.claude-haiku-4-5';
  }

  if (content.length > 200 || COMPLEX_PATTERNS.some(p => p.test(content))) {
    return 'bedrock.claude-opus-4-6';
  }

  return 'bedrock.claude-sonnet-4-6';
}
