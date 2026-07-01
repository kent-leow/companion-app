import type { LLMClient, ChatMessage } from '../llm/index.js';

const MAX_CONTEXT_CHARS = 8000;
const SUMMARIZE_AFTER_TURNS = 4;

export interface ConversationSummary {
  summary: string;
  lastUserMessage: string;
  lastAssistantResponse: string;
  turnCount: number;
}

interface Turn {
  user: string;
  assistant: string;
}

export class ContextManager {
  private turns: Turn[] = [];
  private summary: string = '';
  private llmClient: LLMClient | null;

  constructor(llmClient?: LLMClient) {
    this.llmClient = llmClient ?? null;
  }

  get turnCount(): number {
    return this.turns.length;
  }

  addTurn(userMessage: string, assistantResponse: string): void {
    this.turns.push({ user: userMessage, assistant: assistantResponse });
  }

  async updateSummary(): Promise<void> {
    if (this.turns.length < SUMMARIZE_AFTER_TURNS) return;

    if (this.llmClient) {
      const turnsText = this.turns.slice(0, -2).map((t, i) =>
        `Turn ${i + 1}: User: ${t.user}\nAssistant: ${t.assistant}`
      ).join('\n\n');

      const messages: ChatMessage[] = [
        { role: 'system', content: 'Summarize this conversation in 2-3 sentences. Focus on key facts, decisions, and context needed for future responses. Be concise.' },
        { role: 'user', content: turnsText },
      ];

      const response = await this.llmClient.chat(messages, { model: 'bedrock.claude-haiku-4-5', maxTokens: 300 });
      this.summary = response.content;
    } else {
      this.summary = this.turns.slice(0, -2).map((t, i) =>
        `T${i + 1}: ${t.user.slice(0, 80)} → ${t.assistant.slice(0, 80)}`
      ).join('; ');
    }

    if (this.summary.length > MAX_CONTEXT_CHARS) {
      this.summary = this.summary.slice(0, MAX_CONTEXT_CHARS);
    }
  }

  getContextForPrompt(): string {
    if (this.turns.length === 0) return '';

    if (this.turns.length < SUMMARIZE_AFTER_TURNS) {
      const rawContext = this.turns.map((t, i) =>
        `Turn ${i + 1}:\nUser: ${t.user}\nAssistant: ${t.assistant}`
      ).join('\n\n');
      return truncate(rawContext);
    }

    const parts: string[] = [];
    if (this.summary) {
      parts.push(`Prior conversation summary: ${this.summary}`);
    }

    const recent = this.turns.slice(-2);
    for (const t of recent) {
      parts.push(`User: ${t.user}\nAssistant: ${t.assistant}`);
    }

    return truncate(parts.join('\n\n'));
  }

  getSubAgentContext(): string {
    if (this.turns.length === 0) return '';

    if (this.summary) return this.summary.slice(0, 400);

    const last = this.turns[this.turns.length - 1];
    return `User asked: ${last.user.slice(0, 200)}`;
  }

  getState(): ConversationSummary {
    const lastTurn = this.turns[this.turns.length - 1];
    return {
      summary: this.summary || this.getContextForPrompt(),
      lastUserMessage: lastTurn?.user ?? '',
      lastAssistantResponse: lastTurn?.assistant ?? '',
      turnCount: this.turns.length,
    };
  }
}

function truncate(text: string): string {
  if (text.length <= MAX_CONTEXT_CHARS) return text;
  return text.slice(0, MAX_CONTEXT_CHARS) + '...';
}
