import { useEffect, useRef, useState } from 'react'
import type { TerminalLine, TerminalToken } from '../../data/mockData'
import { terminalSeed } from '../../data/mockData'
import './terminal.css'

const tokenize = (value: string, kind: TerminalToken['kind']): TerminalToken[] =>
  value.split(/(\s+)/).filter(Boolean).map((text) => ({ text, kind: /^\s+$/.test(text) ? 'output' : kind }))

/** Token-first shell renderer: scrollback is structured rather than a single text blob. */
export function Terminal() {
  const [lines, setLines] = useState<TerminalLine[]>(terminalSeed)
  const [input, setInput] = useState('')
  const inputRef = useRef<HTMLInputElement>(null)

  useEffect(() => { inputRef.current?.focus() }, [])

  const execute = () => {
    const command = input.trim()
    if (!command) return
    const commandLine: TerminalLine = { id: crypto.randomUUID(), tokens: [{ text: '$ ', kind: 'prompt' }, ...tokenize(command, 'command')] }
    const response = command === 'help'
      ? [{ text: 'Commands: ', kind: 'muted' as const }, { text: 'help', kind: 'command' as const }, { text: ', ', kind: 'output' as const }, { text: 'mkdir hello', kind: 'command' as const }, { text: ', clear', kind: 'output' as const }]
      : command === 'clear' ? []
      : command.startsWith('mkdir ') ? [{ text: `Created directory: ${command.slice(6)}`, kind: 'success' as const }]
      : [{ text: `${command}: command not found`, kind: 'error' as const }]
    setLines(command === 'clear' ? [] : (current) => [...current, commandLine, { id: crypto.randomUUID(), tokens: response }])
    setInput('')
  }

  return <section className="terminal" aria-label="Interactive learning terminal" onClick={() => inputRef.current?.focus()}>
    <div className="terminal__bar"><span className="terminal__dot" /><span>carbon@learn: ~/chapter-1</span><span className="terminal__status">connected</span></div>
    <div className="terminal__scrollback">
      {lines.map((line) => <div className="terminal__line" key={line.id}>{line.tokens.map((token, index) => <span className={`token token--${token.kind}`} key={`${line.id}-${index}`} title={token.interactive ? 'Documentation available' : undefined}>{token.text}</span>)}</div>)}
      <form className="terminal__input" onSubmit={(event) => { event.preventDefault(); execute() }}>
        <span className="token token--prompt">$&nbsp;</span><input ref={inputRef} value={input} onChange={(event) => setInput(event.target.value)} aria-label="Terminal input" autoComplete="off" spellCheck="false" />
      </form>
    </div>
  </section>
}
