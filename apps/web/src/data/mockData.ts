export type TerminalTokenKind = 'prompt' | 'command' | 'argument' | 'output' | 'success' | 'error' | 'muted'

export interface TerminalToken { text: string; kind: TerminalTokenKind; interactive?: boolean }
export interface TerminalLine { id: string; tokens: TerminalToken[] }

export const terminalSeed: TerminalLine[] = [
  { id: 'welcome', tokens: [{ text: 'Carbon shell — learn by doing.', kind: 'muted' }] },
  { id: 'hint', tokens: [{ text: 'Try ', kind: 'muted' }, { text: 'help', kind: 'command', interactive: true }, { text: ' to see available commands.', kind: 'muted' }] },
]

export const lesson = {
  title: 'Create your first directory',
  description: 'A directory is a container that helps you organise files in a Linux system.',
  objectives: ['Understand the current working directory', 'Create a directory with mkdir', 'Verify it with ls'],
  score: 120,
  progress: 66,
}

export const chapters = [
  { title: 'Chapter 1 · Getting around', progress: 66, lessons: ['The command line', 'Create a directory', 'Move and rename'] },
  { title: 'Chapter 2 · Files and permissions', progress: 0, lessons: ['Read a file', 'Who can access it?'] },
]
