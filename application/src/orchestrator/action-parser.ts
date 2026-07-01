export interface ParsedAction {
  type: 'TOOL' | 'SUBAGENT' | 'RESPOND' | 'THINK';
  target?: string;
  params?: Record<string, unknown>;
  content?: string;
}

export function parseActions(llmOutput: string): ParsedAction[] {
  const actions: ParsedAction[] = [];
  const lines = llmOutput.split('\n');
  let i = 0;

  while (i < lines.length) {
    const line = lines[i];

    if (line.startsWith('[TOOL:')) {
      const match = line.match(/^\[TOOL:([^\]]+)\]\s*(.+)$/);
      if (match) {
        try {
          actions.push({
            type: 'TOOL',
            target: match[1],
            params: JSON.parse(match[2]),
          });
        } catch {
          actions.push({
            type: 'TOOL',
            target: match[1],
            params: { raw: match[2] },
          });
        }
      }
    } else if (line.startsWith('[SUBAGENT:')) {
      const match = line.match(/^\[SUBAGENT:([^\]]+)\]\s*(.+)$/);
      if (match) {
        try {
          actions.push({
            type: 'SUBAGENT',
            target: match[1],
            params: JSON.parse(match[2]),
          });
        } catch {
          actions.push({
            type: 'SUBAGENT',
            target: match[1],
            params: { raw: match[2] },
          });
        }
      }
    } else if (line.startsWith('[RESPOND]')) {
      const contentLines: string[] = [line.slice('[RESPOND] '.length)];
      i++;
      while (i < lines.length && !lines[i].startsWith('[')) {
        contentLines.push(lines[i]);
        i++;
      }
      actions.push({
        type: 'RESPOND',
        content: contentLines.join('\n').trim(),
      });
      continue;
    } else if (line.startsWith('[THINK]')) {
      const contentLines: string[] = [line.slice('[THINK] '.length)];
      i++;
      while (i < lines.length && !lines[i].startsWith('[')) {
        contentLines.push(lines[i]);
        i++;
      }
      actions.push({
        type: 'THINK',
        content: contentLines.join('\n').trim(),
      });
      continue;
    }

    i++;
  }

  return actions;
}
