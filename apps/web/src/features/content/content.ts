const docs = import.meta.glob('../../../../../content/docs/*.yaml', { eager: true, query: '?raw', import: 'default' }) as Record<string, string>
const glossary = import.meta.glob('../../../../../content/glossary/*.md', { eager: true, query: '?raw', import: 'default' }) as Record<string, string>
const mascot = import.meta.glob('../../../../../content/mascot/*.json', { eager: true, query: '?raw', import: 'default' }) as Record<string, string>

type Parameter = { name: string; required?: boolean; description: string }
type Example = { command: string; output?: string; description?: string }
type Mistake = { mistake: string; explanation: string }
type DocumentError = { name: string; condition: string }
export type CommandDoc = { name: string; summary: string; purpose: string; syntax: string[]; parameters: Parameter[]; examples: Example[]; result: string; notes: string[]; commonMistakes: Mistake[]; errors: DocumentError[]; limitations: string[]; related: string[] }

const unquote = (value: string) => value.trim().replace(/^['"]|['"]$/g, '')
const oneLine = (value: string) => unquote(value.replace(/^\s*[>|]\s*/, '').replace(/\n\s*/g, ' '))
const section = (source: string, name: string, until: string[]) => source.match(new RegExp(`^\\s*${name}:\\s*(.*?)(?=^\\s*(?:${until.join('|')}):|(?![\\s\\S]))`, 'ms'))?.[1] ?? ''
const list = (source: string) => source.split('\n').filter((line) => /^\s*- (?!name:|command:|mistake:)/.test(line)).map((line) => unquote(line.replace(/^\s*- /, '')))

function parseDocument(source: string, path: string): CommandDoc {
  const value = (name: string, until: string[]) => section(source, name, until)
  const parameters = [...source.matchAll(/- name:\s*["']?([^\n"']+)["']?[\s\S]*?description:\s*(?:>\s*\n)?\s*["']?([^\n"']+(?:\n\s+[^\n]+)*)/g)].map((item) => ({ name: unquote(item[1]), required: /required:\s*true/.test(item[0]), description: oneLine(item[2]) }))
  const examples = [...source.matchAll(/- command:\s*(?:\|\s*\n)?\s*["']?([^\n"']+)["']?([\s\S]*?)(?=\n\s*- command:|\n\s*result:)/g)].map((item) => ({ command: unquote(item[1]), output: item[2].match(/output:\s*\|?\s*\n?\s*([^\n]*(?:\n\s+[^\n]*)*)/)?.[1].trim(), description: item[2].match(/description:\s*["']?([^\n"']+)/)?.[1] }))
  const commonMistakes = [...source.matchAll(/- mistake:\s*["']?([^\n"']+)["']?[\s\S]*?explanation:\s*(?:>\s*\n)?\s*["']?([^\n"']+(?:\n\s+[^\n]+)*)/g)].map((item) => ({ mistake: unquote(item[1]), explanation: oneLine(item[2]) }))
  const errors = [...source.matchAll(/- name:\s*["']?([^\n"']+)["']?[\s\S]*?condition:\s*["']?([^\n"']+)/g)].map((item) => ({ name: unquote(item[1]), condition: oneLine(item[2]) }))
  return {
    name: path.split('/').pop()!.replace('.yaml', ''), summary: oneLine(value('summary', ['purpose'])), purpose: oneLine(value('purpose', ['syntax'])),
    syntax: list(value('syntax', ['parameters'])), parameters, examples, result: oneLine(value('result', ['notes'])),
    notes: list(value('notes', ['common_mistakes'])), commonMistakes, errors, limitations: list(value('limitations', ['status'])), related: list(value('related_commands', [])),
  }
}

export const commandDocs = Object.entries(docs).map(([path, source]) => parseDocument(source, path)).sort((left, right) => left.name.localeCompare(right.name))
export const getCommandDoc = (name: string) => commandDocs.find((doc) => doc.name === name)
export const glossaryEntries = Object.entries(glossary).map(([path, markdown]) => ({ id: path.split('/').pop()!.replace('.md', ''), markdown }))
export const getGlossaryEntry = (id: string) => glossaryEntries.find((entry) => entry.id === id)
export const mascotDialogues = Object.values(mascot).map((source) => JSON.parse(source) as { command: string; dialogue: Record<string, string[]> })
export const getMascotDialogue = (command?: string) => mascotDialogues.find((entry) => entry.command === command)
