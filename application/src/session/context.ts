import { ContextManager } from '../orchestrator/context-manager.js';
import { MemoryManager } from '../memory/index.js';
import type { LLMClient } from '../llm/index.js';

export interface SessionContext {
  contextManager: ContextManager;
  memory: MemoryManager;
}

export function createSessionContext(
  memoryPath: string,
  llmClient?: LLMClient,
): SessionContext {
  return {
    contextManager: new ContextManager(llmClient),
    memory: new MemoryManager(memoryPath),
  };
}
