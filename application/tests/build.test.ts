import { describe, it, expect } from 'vitest';
import { existsSync, statSync } from 'node:fs';
import { resolve } from 'node:path';

const DIST = resolve(import.meta.dirname, '../dist');

describe('Build', () => {
  it('dist/companion.js exists', () => {
    expect(existsSync(resolve(DIST, 'companion.js'))).toBe(true);
  });

  it('bundle size is under 5MB', () => {
    const stats = statSync(resolve(DIST, 'companion.js'));
    expect(stats.size).toBeLessThan(5 * 1024 * 1024);
  });

  it('has shebang for CLI execution', async () => {
    const { readFileSync } = await import('node:fs');
    const content = readFileSync(resolve(DIST, 'companion.js'), 'utf-8');
    expect(content.startsWith('#!/usr/bin/env node')).toBe(true);
  });

  it('sourcemap exists', () => {
    expect(existsSync(resolve(DIST, 'companion.js.map'))).toBe(true);
  });
});
