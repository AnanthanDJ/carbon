import { useEffect, useRef, useState } from "react";
import type { TerminalLine, TerminalToken } from "../../data/mockData";
import { terminalSeed } from "../../data/mockData";
import { api, ApiError } from '../../api/client'
import { useLearning } from '../lesson/LearningProvider'
import { useAuth } from '../../auth/AuthProvider'
import "./terminal.css";

const tokenize = (
  value: string,
  kind: TerminalToken["kind"],
): TerminalToken[] =>
  value
    .split(/(\s+)/)
    .filter(Boolean)
    .map((text) => ({ text, kind: /^\s+$/.test(text) ? "output" : kind }));

const createLine = (tokens: TerminalToken[]): TerminalLine => ({
  id: crypto.randomUUID(),
  tokens,
});

/** Token-first shell renderer: scrollback is structured rather than a single text blob. */
export function Terminal() {
  const [lines, setLines] = useState<TerminalLine[]>(terminalSeed);
  const [input, setInput] = useState("");
  const [isExecuting, setIsExecuting] = useState(false);
  const { cwd, setCwd, applyOutcome, refreshLesson } = useLearning()
  const inputRef = useRef<HTMLInputElement>(null);
  const scrollbackRef = useRef<HTMLDivElement>(null);
  const { user } = useAuth();

  useEffect(() => {
    if (!isExecuting) inputRef.current?.focus();
  }, [isExecuting]);
  useEffect(() => {
    scrollbackRef.current?.scrollTo({
      top: scrollbackRef.current.scrollHeight,
    });
  }, [lines, isExecuting]);

  const execute = async () => {
    const command = input;
    if (!command || isExecuting) return;

    const commandLine = createLine([
      { text: "$ ", kind: "prompt" },
      ...tokenize(command, "command"),
    ]);
    setInput("");
    setIsExecuting(true);

    try {
      const result = await api.execute(command, cwd)
      setCwd(result.cwd);
      applyOutcome(result.lesson)
      void refreshLesson()
      if (command === "clear") {
        setLines([]);
      } else {
        setLines((current) => [
          ...current,
          commandLine,
          ...(result.stdout
            ? [createLine([{ text: result.stdout, kind: "output" }])]
            : []),
          ...(result.stderr
            ? [createLine([{ text: result.stderr, kind: "error" }])]
            : []),
        ]);
      }
    } catch (error) {
      const message = error instanceof ApiError && error.status === 400 ? 'Command could not be completed.' : error instanceof Error ? error.message : "Unable to reach the terminal service.";
      setLines((current) => [
        ...current,
        commandLine,
        createLine([{ text: message, kind: "error" }]),
      ]);
    } finally {
      setIsExecuting(false);
      inputRef.current?.focus();
    }
  };

  return (
    <section
      className="terminal"
      aria-label="Interactive learning terminal"
      onClick={() => inputRef.current?.focus()}
    >
      <div className="terminal__bar">
        <span className="terminal__dot" />
        <span>
          {user?.username ?? "guest"}@carbon: {cwd}
        </span>
        <span className="terminal__status">
          {isExecuting ? "running…" : "connected"}
        </span>
      </div>
      <div className="terminal__scrollback" ref={scrollbackRef}>
        {lines.map((line) => (
          <div className="terminal__line" key={line.id}>
            {line.tokens.map((token, index) => (
              <span
                className={`token token--${token.kind}`}
                key={`${line.id}-${index}`}
                title={
                  token.interactive ? "Documentation available" : undefined
                }
              >
                {token.text}
              </span>
            ))}
          </div>
        ))}
        <form
          className="terminal__input"
          onSubmit={(event) => {
            event.preventDefault();
            execute();
          }}
        >
          <span className="token token--prompt">
            {user?.username ?? "guest"}@carbon $&nbsp;
          </span>
          <input
            ref={inputRef}
            value={input}
            onChange={(event) => setInput(event.target.value)}
            aria-label="Terminal input"
            autoComplete="off"
            spellCheck="false"
            disabled={isExecuting}
          />
        </form>
      </div>
    </section>
  );
}
