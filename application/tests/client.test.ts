import { describe, it, expect, vi, beforeEach } from 'vitest';
import { LLMClient } from '../src/llm/client.js';

describe('LLMClient', () => {
  let client: LLMClient;

  beforeEach(() => {
    client = new LLMClient('https://api.example.com', 'test-key');
  });

  it('sends correct headers and body format', async () => {
    const mockResponse = {
      ok: true,
      json: () => Promise.resolve({
        choices: [{ message: { content: 'Hello' } }],
        model: 'bedrock.claude-sonnet-4-6',
        usage: { prompt_tokens: 10, completion_tokens: 5, total_tokens: 15 },
      }),
    };
    vi.spyOn(globalThis, 'fetch').mockResolvedValueOnce(mockResponse as any);

    await client.chat([{ role: 'user', content: 'Hi' }]);

    expect(fetch).toHaveBeenCalledWith(
      'https://api.example.com/v1/chat/completions',
      expect.objectContaining({
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'x-api-key': 'test-key',
        },
      }),
    );

    const body = JSON.parse((fetch as any).mock.calls[0][1].body);
    expect(body.messages).toEqual([{ role: 'user', content: 'Hi' }]);
    expect(body.max_tokens).toBe(4096);
    expect(body.stream).toBe(false);
  });

  it('returns structured response for non-streaming', async () => {
    const mockResponse = {
      ok: true,
      json: () => Promise.resolve({
        choices: [{ message: { content: 'The answer is 42' } }],
        model: 'bedrock.claude-sonnet-4-6',
        usage: { prompt_tokens: 10, completion_tokens: 8, total_tokens: 18 },
      }),
    };
    vi.spyOn(globalThis, 'fetch').mockResolvedValueOnce(mockResponse as any);

    const result = await client.chat([{ role: 'user', content: 'What is the answer?' }]);
    expect(result).toEqual({
      content: 'The answer is 42',
      model: 'bedrock.claude-sonnet-4-6',
      usage: { prompt_tokens: 10, completion_tokens: 8, total_tokens: 18 },
    });
  });

  it('throws on API error', async () => {
    vi.spyOn(globalThis, 'fetch').mockResolvedValueOnce({
      ok: false,
      status: 429,
      text: () => Promise.resolve('Rate limited'),
    } as any);

    await expect(
      client.chat([{ role: 'user', content: 'Hi' }])
    ).rejects.toThrow('LLM API error 429: Rate limited');
  });
});
