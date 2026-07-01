import { describe, it, expect, beforeEach, afterEach } from 'vitest';
import { mkdtempSync, rmSync } from 'node:fs';
import { join } from 'node:path';
import { tmpdir } from 'node:os';
import { MemoryManager } from '../src/memory/index.js';

describe('MemoryManager', () => {
  let tempDir: string;
  let manager: MemoryManager;

  beforeEach(() => {
    tempDir = mkdtempSync(join(tmpdir(), 'mem-mgr-'));
    manager = new MemoryManager(join(tempDir, 'memory.md'));
  });

  afterEach(() => {
    rmSync(tempDir, { recursive: true });
  });

  it('appends entries and reads them back', () => {
    manager.append('Remember: user prefers TypeScript');
    const content = manager.read();
    expect(content).toContain('user prefers TypeScript');
  });

  it('auto-prunes when over budget', () => {
    for (let i = 0; i < 200; i++) {
      manager.append(`Entry ${i}: ${'data'.repeat(100)}`);
    }
    expect(manager.getTokenCount()).toBeLessThanOrEqual(8000);
  });
});
