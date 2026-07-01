import { readFileSync, writeFileSync, existsSync, mkdirSync } from 'node:fs';
import { dirname } from 'node:path';

export class MemoryStore {
  private filePath: string;

  constructor(filePath: string) {
    this.filePath = filePath;
  }

  read(): string {
    if (!existsSync(this.filePath)) return '';
    return readFileSync(this.filePath, 'utf-8');
  }

  write(content: string): void {
    const dir = dirname(this.filePath);
    if (!existsSync(dir)) mkdirSync(dir, { recursive: true });
    writeFileSync(this.filePath, content, 'utf-8');
  }

  append(entry: string): void {
    const current = this.read();
    const timestamp = new Date().toISOString().slice(0, 16);
    const newEntry = `\n## [${timestamp}]\n${entry}\n`;
    this.write(current + newEntry);
  }
}
