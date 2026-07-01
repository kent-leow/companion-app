import { parseSSEStream } from './streaming.js';
import { selectModel, type ModelId } from './model-selector.js';

export interface ChatMessage {
  role: 'system' | 'user' | 'assistant';
  content: string | ContentPart[];
}

export interface ContentPart {
  type: 'text' | 'image_url';
  text?: string;
  image_url?: { url: string };
}

export interface ChatOptions {
  model?: ModelId;
  stream?: boolean;
  maxTokens?: number;
  temperature?: number;
}

export interface ChatResponse {
  content: string;
  model: string;
  usage?: { prompt_tokens: number; completion_tokens: number; total_tokens: number };
}

export class LLMClient {
  private baseUrl: string;
  private authToken: string;

  constructor(baseUrl: string, authToken: string) {
    this.baseUrl = baseUrl.replace(/\/$/, '');
    this.authToken = authToken;
  }

  async chat(messages: ChatMessage[], options: ChatOptions & { stream: true }): Promise<AsyncIterable<string>>;
  async chat(messages: ChatMessage[], options?: ChatOptions): Promise<ChatResponse>;
  async chat(messages: ChatMessage[], options: ChatOptions = {}): Promise<ChatResponse | AsyncIterable<string>> {
    const model = options.model ?? selectModel(messages);
    const maxTokens = options.maxTokens ?? 4096;

    const body = {
      model,
      messages,
      max_tokens: maxTokens,
      stream: options.stream ?? false,
      ...(options.temperature !== undefined && { temperature: options.temperature }),
    };

    const response = await fetch(`${this.baseUrl}/v1/chat/completions`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-api-key': this.authToken,
      },
      body: JSON.stringify(body),
    });

    if (!response.ok) {
      const text = await response.text();
      throw new Error(`LLM API error ${response.status}: ${text}`);
    }

    if (options.stream) {
      return parseSSEStream(response.body!);
    }

    const data = await response.json() as any;
    return {
      content: data.choices[0].message.content,
      model: data.model,
      usage: data.usage,
    };
  }
}
