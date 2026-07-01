import { describe, it, expect, beforeEach, afterEach } from 'vitest';
import { loadEnv } from '../src/config/env.js';

describe('loadEnv', () => {
  const originalEnv = process.env;

  beforeEach(() => {
    process.env = { ...originalEnv };
  });

  afterEach(() => {
    process.env = originalEnv;
  });

  it('throws when ANTHROPIC_BASE_URL is missing', () => {
    delete process.env.ANTHROPIC_BASE_URL;
    process.env.ANTHROPIC_AUTH_TOKEN = 'test-token';
    expect(() => loadEnv()).toThrow('Missing required env var: ANTHROPIC_BASE_URL');
  });

  it('throws when ANTHROPIC_AUTH_TOKEN is missing', () => {
    process.env.ANTHROPIC_BASE_URL = 'https://api.example.com';
    delete process.env.ANTHROPIC_AUTH_TOKEN;
    expect(() => loadEnv()).toThrow('Missing required env var: ANTHROPIC_AUTH_TOKEN');
  });

  it('returns typed config when all vars present', () => {
    process.env.ANTHROPIC_BASE_URL = 'https://api.example.com';
    process.env.ANTHROPIC_AUTH_TOKEN = 'test-token';
    const config = loadEnv();
    expect(config).toEqual({
      baseUrl: 'https://api.example.com',
      authToken: 'test-token',
    });
  });
});
