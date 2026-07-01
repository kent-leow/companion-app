import { MemoryStore } from './store.js';
import { pruneMemory, estimateTokens } from './pruner.js';

export class MemoryManager {
  private store: MemoryStore;

  constructor(filePath: string) {
    this.store = new MemoryStore(filePath);
  }

  read(): string {
    return this.store.read();
  }

  append(entry: string): void {
    this.store.append(entry);
    this.prune();
  }

  prune(): void {
    const content = this.store.read();
    const pruned = pruneMemory(content);
    if (pruned !== content) {
      this.store.write(pruned);
    }
  }

  getTokenCount(): number {
    return estimateTokens(this.store.read());
  }
}

export { MemoryStore } from './store.js';
export { pruneMemory, estimateTokens } from './pruner.js';
