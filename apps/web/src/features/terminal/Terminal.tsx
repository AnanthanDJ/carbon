import { useEffect, useRef, useState } from 'react'
import type { TerminalLine, TerminalToken } from '../../data/mockData'
import { terminalSeed } from '../../data/mockData'
import './terminal.css'

const tokenize = (value: string, kind: TerminalToken['kind']): TerminalToken[] =>
  value.split(/(\s+)/).filter(Boolean).map((text) => ({ text, kind: /^\s+$/.test(text) ? 'output' : kind }))

interface TerminalResponse {
  stdout: string
  cwd: string
}

const createLine = (tokens: TerminalToken[]): TerminalLine => ({ id: crypto.randomUUID(), tokens })

/** Token-first shell renderer: scrollback is structured rather than a single text blob. */
export function Terminal() {
  const [lines, setLines] = useState<TerminalLine[]>(terminalSeed)
  const [input, setInput] = useState('')
  const [cwd, setCwd] = useState('/')
  const [isExecuting, setIsExecuting] = useState(false)
  const inputRef = useRef<HTMLInputElement>(null)
  const scrollbackRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    if (!isExecuting) inputRef.current?.focus()
  }, [isExecuting])
  useEffect(() => { scrollbackRef.current?.scrollTo({ top: scrollbackRef.current.scrollHeight }) }, [lines, isExecuting])

  const execute = async () => {
    const command = input
    if (!command || isExecuting) return

    const commandLine = createLine([{ text: '$ ', kind: 'prompt' }, ...tokenize(command, 'command')])
    setInput('')
    setIsExecuting(true)

    try {
      const response = await fetch('/api/terminal', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ command, cwd }),
      })

      if (!response.ok) {
        throw new Error(response.status === 400 ? 'Command could not be completed.' : 'Unable to execute command.')
      }

      const result: TerminalResponse = await response.json()
      setCwd(result.cwd)
      if (command === 'clear') {
        setLines([])
      } else {
        setLines((current) => [
          ...current,
          commandLine,
          ...(result.stdout ? [createLine([{ text: result.stdout, kind: 'output' }])] : []),
        ])
      }
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Unable to reach the terminal service.'
      setLines((current) => [...current, commandLine, createLine([{ text: message, kind: 'error' }])])
    } finally {
      setIsExecuting(false)
      inputRef.current?.focus()
    }
  }

  return <section className="terminal" aria-label="Interactive learning terminal" onClick={() => inputRef.current?.focus()}>
    <div className="terminal__bar"><span className="terminal__dot" /><span>carbon@learn: {cwd}</span><span className="terminal__status">{isExecuting ? 'running…' : 'connected'}</span></div>
    <div className="terminal__scrollback" ref={scrollbackRef}>
      {lines.map((line) => <div className="terminal__line" key={line.id}>{line.tokens.map((token, index) => <span className={`token token--${token.kind}`} key={`${line.id}-${index}`} title={token.interactive ? 'Documentation available' : undefined}>{token.text}</span>)}</div>)}
      <form className="terminal__input" onSubmit={(event) => { event.preventDefault(); execute() }}>
        <span className="token token--prompt">$&nbsp;</span><input ref={inputRef} value={input} onChange={(event) => setInput(event.target.value)} aria-label="Terminal input" autoComplete="off" spellCheck="false" disabled={isExecuting} />
      </form>
    </div>
  </section>
}
