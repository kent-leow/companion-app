import { randomBytes } from 'node:crypto';
import { join } from 'node:path';
import { createSessionContext, type SessionContext } from './context.js';
import type { LLMClient } from '../llm/index.js';

export interface Session {
  id: string;
  context: SessionContext;
  createdAt: Date;
}

export class SessionManager {
  private sessions: Map<string, Session> = new Map();
  private dataDir: string;
  private llmClient?: LLMClient;

  constructor(dataDir: string, llmClient?: LLMClient) {
    this.dataDir = dataDir;
    this.llmClient = llmClient;
  }

  create(): Session {
    const id = randomBytes(8).toString('hex');
    const memoryPath = join(this.dataDir, 'sessions', id, 'memory.md');
    const context = createSessionContext(memoryPath, this.llmClient);
    const session: Session = { id, context, createdAt: new Date() };
    this.sessions.set(id, session);
    return session;
  }

  get(id: string): Session | undefined {
    return this.sessions.get(id);
  }

  list(): Session[] {
    return Array.from(this.sessions.values());
  }

  destroy(id: string): boolean {
    return this.sessions.delete(id);
  }
}

export { createSessionContext, type SessionContext } from './context.js';
