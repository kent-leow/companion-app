import { describe, it, expect, beforeEach, afterEach } from 'vitest';
import { mkdtempSync, rmSync } from 'node:fs';
import { join } from 'node:path';
import { tmpdir } from 'node:os';
import { SessionManager } from '../src/session/index.js';

describe('SessionManager', () => {
  let tempDir: string;
  let manager: SessionManager;

  beforeEach(() => {
    tempDir = mkdtempSync(join(tmpdir(), 'session-test-'));
    manager = new SessionManager(tempDir);
  });

  afterEach(() => {
    rmSync(tempDir, { recursive: true });
  });

  it('creates session with unique ID', () => {
    const session = manager.create();
    expect(session.id).toHaveLength(16);
    expect(session.createdAt).toBeInstanceOf(Date);
    expect(session.context.contextManager).toBeDefined();
    expect(session.context.memory).toBeDefined();
  });

  it('multiple sessions have independent state', () => {
    const s1 = manager.create();
    const s2 = manager.create();
    expect(s1.id).not.toBe(s2.id);

    s1.context.contextManager.addTurn('Hello', 'Hi');
    expect(s1.context.contextManager.turnCount).toBe(1);
    expect(s2.context.contextManager.turnCount).toBe(0);
  });

  it('sessions share nothing between instances', () => {
    const s1 = manager.create();
    const s2 = manager.create();
    s1.context.memory.append('Only for session 1');
    expect(s2.context.memory.read()).toBe('');
  });

  it('lists all sessions', () => {
    manager.create();
    manager.create();
    expect(manager.list()).toHaveLength(2);
  });

  it('gets session by ID', () => {
    const session = manager.create();
    expect(manager.get(session.id)).toBe(session);
    expect(manager.get('nonexistent')).toBeUndefined();
  });

  it('destroys session', () => {
    const session = manager.create();
    expect(manager.destroy(session.id)).toBe(true);
    expect(manager.get(session.id)).toBeUndefined();
    expect(manager.list()).toHaveLength(0);
  });
});
