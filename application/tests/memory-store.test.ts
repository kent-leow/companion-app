import { describe, it, expect, beforeEach, afterEach } from 'vitest';
import { mkdtempSync, rmSync } from 'node:fs';
import { join } from 'node:path';
import { tmpdir } from 'node:os';
import { MemoryStore } from '../src/memory/store.js';

describe('MemoryStore', () => {
  let tempDir: string;

  beforeEach(() => {
    tempDir = mkdtempSync(join(tmpdir(), 'mem-test-'));
  });

  afterEach(() => {
    rmSync(tempDir, { recursive: true });
  });

  it('returns empty string for non-existent file', () => {
    const store = new MemoryStore(join(tempDir, 'memory.md'));
    expect(store.read()).toBe('');
  });

  it('writes and reads content', () => {
    const store = new MemoryStore(join(tempDir, 'memory.md'));
    store.write('# Memory\nSome content');
    expect(store.read()).toBe('# Memory\nSome content');
  });

  it('appends timestamped entries', () => {
    const store = new MemoryStore(join(tempDir, 'memory.md'));
    store.append('First entry');
    store.append('Second entry');
    const content = store.read();
    expect(content).toContain('First entry');
    expect(content).toContain('Second entry');
    expect(content).toMatch(/## \[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}\]/);
  });

  it('creates parent directories if missing', () => {
    const store = new MemoryStore(join(tempDir, 'nested/dir/memory.md'));
    store.write('test');
    expect(store.read()).toBe('test');
  });
});
