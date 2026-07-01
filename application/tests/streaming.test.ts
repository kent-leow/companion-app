import { describe, it, expect } from 'vitest';
import { parseSSEStream } from '../src/llm/streaming.js';

function createMockStream(chunks: string[]): ReadableStream<Uint8Array> {
  const encoder = new TextEncoder();
  return new ReadableStream({
    start(controller) {
      for (const chunk of chunks) {
        controller.enqueue(encoder.encode(chunk));
      }
      controller.close();
    },
  });
}

describe('parseSSEStream', () => {
  it('yields tokens from SSE data chunks', async () => {
    const stream = createMockStream([
      'data: {"choices":[{"delta":{"content":"Hello"}}]}\n\n',
      'data: {"choices":[{"delta":{"content":" world"}}]}\n\n',
      'data: [DONE]\n\n',
    ]);

    const tokens: string[] = [];
    for await (const token of parseSSEStream(stream)) {
      tokens.push(token);
    }
    expect(tokens).toEqual(['Hello', ' world']);
  });

  it('handles split chunks across boundaries', async () => {
    const stream = createMockStream([
      'data: {"choices":[{"delta":{"con',
      'tent":"Hi"}}]}\n\ndata: [DONE]\n\n',
    ]);

    const tokens: string[] = [];
    for await (const token of parseSSEStream(stream)) {
      tokens.push(token);
    }
    expect(tokens).toEqual(['Hi']);
  });

  it('skips empty delta content', async () => {
    const stream = createMockStream([
      'data: {"choices":[{"delta":{}}]}\n\n',
      'data: {"choices":[{"delta":{"content":"ok"}}]}\n\n',
      'data: [DONE]\n\n',
    ]);

    const tokens: string[] = [];
    for await (const token of parseSSEStream(stream)) {
      tokens.push(token);
    }
    expect(tokens).toEqual(['ok']);
  });

  it('skips malformed JSON', async () => {
    const stream = createMockStream([
      'data: {invalid json}\n\n',
      'data: {"choices":[{"delta":{"content":"valid"}}]}\n\n',
      'data: [DONE]\n\n',
    ]);

    const tokens: string[] = [];
    for await (const token of parseSSEStream(stream)) {
      tokens.push(token);
    }
    expect(tokens).toEqual(['valid']);
  });
});
