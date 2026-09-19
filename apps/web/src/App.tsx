import { useEffect, useMemo, useRef, useState } from "react";
import {
  Link,
  NavLink,
  Navigate,
  Route,
  Routes,
  useNavigate,
  useParams,
} from "react-router-dom";
import { ApiError } from "./api/client";
import { useAuth } from "./auth/AuthProvider";
import { Markdown } from "./features/content/Markdown";
import {
  commandDocs,
  getCommandDoc,
  getGlossaryEntry,
} from "./features/content/content";
import { useLearning } from "./features/lesson/LearningProvider";
import { Terminal } from "./features/terminal/Terminal";
import { themes } from "./themes";
import { useTheme } from "./themes/themeContext";
import {
  Terminal as HeroTerminal,
  TerminalHeader,
  TerminalTitle,
  TerminalContent,
} from "@/components/ai/terminal";
import "./features/settings/themeSettings.css";
import "./App.css";
import { AsciiFluid } from "@/components/ui/ascii-fluid";

function Protected({ children }: { children: React.ReactNode }) {
  const { user, ready } = useAuth();
  if (!ready) return <main className="auth-page">Restoring your session…</main>;
  return user ? <>{children}</> : <Navigate to="/login" replace />;
}

function AuthPage({ mode }: { mode: "login" | "register" }) {
  const { user, authenticate } = useAuth();
  const navigate = useNavigate();
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const [busy, setBusy] = useState(false);
  if (user) return <Navigate to="/app" replace />;
  const submit = async (event: React.FormEvent) => {
    event.preventDefault();
    setError("");
    setBusy(true);
    try {
      await authenticate(username, password, mode);
      navigate("/app");
    } catch (reason) {
      setError(
        reason instanceof ApiError
          ? reason.message
          : "Unable to connect to Carbon.",
      );
    } finally {
      setBusy(false);
    }
  };
  return (
    <main className="auth-page">
      <form className="auth-card" onSubmit={submit}>
        <Link className="brand" to="/">
          <span className="brand__mark">C</span>carbon
        </Link>
        <p className="eyebrow">
          {mode === "login" ? "Welcome back" : "Start learning"}
        </p>
        <h1>{mode === "login" ? "Log in" : "Create your account"}</h1>
        <label>
          Username
          <input
            value={username}
            onChange={(event) => setUsername(event.target.value)}
            autoComplete="username"
            required
          />
        </label>
        <label>
          Password
          <input
            type="password"
            value={password}
            onChange={(event) => setPassword(event.target.value)}
            autoComplete={
              mode === "login" ? "current-password" : "new-password"
            }
            required
          />
        </label>
        {error && (
          <p className="form-error" role="alert">
            {error}
          </p>
        )}
        <button className="button" disabled={busy}>
          {busy
            ? "Please wait…"
            : mode === "login"
              ? "Log in"
              : "Create account"}
        </button>
        <p>
          {mode === "login" ? "New to Carbon?" : "Already have an account?"}{" "}
          <Link to={mode === "login" ? "/register" : "/login"}>
            {mode === "login" ? "Register" : "Log in"}
          </Link>
        </p>
      </form>
    </main>
  );
}

type IconName = "home" | "lesson" | "terminal" | "docs";

const navigation: { label: string; path: string; icon: IconName }[] = [
  { label: "Home", path: "/", icon: "home" },
  { label: "Lesson", path: "/app/learn/first-directory", icon: "lesson" },
  { label: "Terminal", path: "/app/terminal", icon: "terminal" },
  { label: "Docs", path: "/app/docs", icon: "docs" },
];

function Header() {
  const { user, logout } = useAuth();

  const [menuOpen, setMenuOpen] = useState(false);
  const [profileOpen, setProfileOpen] = useState(false);

  const profileRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    function handleClick(event: MouseEvent) {
      if (
        profileOpen &&
        profileRef.current &&
        !profileRef.current.contains(event.target as Node)
      ) {
        setProfileOpen(false);
      }
    }

    document.addEventListener("mousedown", handleClick);
    return () => document.removeEventListener("mousedown", handleClick);
  }, [profileOpen]);

  useEffect(() => {
    function handleKey(event: KeyboardEvent) {
      if (event.key === "Escape") {
        setProfileOpen(false);
      }
    }

    document.addEventListener("keydown", handleKey);
    return () => document.removeEventListener("keydown", handleKey);
  }, []);

  return (
    <header className="header">
      <button
        className="menu-button"
        aria-label="Open navigation"
        onClick={() => setMenuOpen(true)}
      >
        ☰
      </button>

      <NavLink className="brand" to="/app/learn/first-directory">
        <span className="brand__mark">C</span>
        <span>carbon</span>
      </NavLink>

      <nav
        className={`nav ${menuOpen ? "nav--open" : ""}`}
        aria-label="Main navigation"
      >
        <button
          className="nav__close"
          onClick={() => setMenuOpen(false)}
          aria-label="Close navigation"
        >
          ×
        </button>

        {navigation.map((item) => (
          <NavLink
            key={item.path}
            to={item.path}
            onClick={() => setMenuOpen(false)}
            className={({ isActive }) =>
              `nav__link ${isActive ? "nav__link--active" : ""}`
            }
          >
            {item.label}
          </NavLink>
        ))}
      </nav>

      {menuOpen && (
        <button
          className="backdrop"
          aria-label="Close navigation"
          onClick={() => setMenuOpen(false)}
        />
      )}

      <div ref={profileRef} className="profile">
        <button
          className="profile__trigger"
          onClick={() => setProfileOpen((value) => !value)}
          aria-expanded={profileOpen}
        >
          <span className="avatar">
            {user?.username.slice(0, 2).toUpperCase()}
          </span>

          <span className="profile__name">{user?.username}</span>

          <span>⌄</span>
        </button>

        {profileOpen && (
          <div className="profile__menu">
            <NavLink to="/app/profile" onClick={() => setProfileOpen(false)}>
              Profile
            </NavLink>

            <NavLink to="/app/settings" onClick={() => setProfileOpen(false)}>
              Settings
            </NavLink>

            <NavLink to="/app/about" onClick={() => setProfileOpen(false)}>
              About
            </NavLink>

            <button
              onClick={() => {
                setProfileOpen(false);
                logout();
              }}
            >
              Log out
            </button>
          </div>
        )}
      </div>
    </header>
  );
}

function HeroPage() {
  const { user } = useAuth();
  const navigate = useNavigate();

  return (
    <main className="hero__landing">
      <AsciiFluid
        theme="dark"
        color="#5cff8d"
        backgroundColor="#050505"
        cellSize={10}
        interactive
        animate
        className="hero__ascii"
      />

      <div className="landing__scroll">
        <button
          className="hero__login"
          onClick={() => navigate(user ? "/app" : "/login")}
        >
          {user ? "Open Carbon" : "Sign In"}
        </button>

        <section className="hero__intro">
          <img
            src="/logo.svg"
            alt="Carbon"
            className="hero__logo"
            draggable={false}
          />

          <p className="hero__subtitle">Learn Linux through the terminal.</p>

              <div className="scroll-cue"><span>Scroll</span><span className="stick"></span></div>

        </section>

        <section className="hero__about">
          <div className="hero__text">
            <p className="hero__eyebrow">Carbon Academy</p>

            <h1>
              Learn by using a terminal.
              <br />
              Not by watching videos.
            </h1>

            <p>
              Carbon teaches Linux through interactive lessons, a real terminal,
              and instant feedback. Every command you type moves your progress
              forward.
            </p>

            <div className="hero__actions">
              <button
                className="button button--primary"
                onClick={() => navigate(user ? "/app" : "/register")}
              >
                {user ? "Continue Learning" : "Start Learning"}
              </button>

              <button className="button" onClick={() => navigate("/app/docs")}>
                Documentation
              </button>
            </div>
          </div>

          <div className="hero__terminal">
            <HeroTerminal className="h-[520px] w-full">
              <TerminalHeader>
                <TerminalTitle>carbon</TerminalTitle>
              </TerminalHeader>

              <TerminalContent>
                <div className="space-y-2 font-mono text-sm">
                  <div>
                    <span className="text-zinc-500">$</span>{" "}
                    <span className="text-green-400">carbon start</span>
                  </div>

                  <div className="text-zinc-400">
                    Initializing interactive Linux environment...
                  </div>

                  <div className="text-emerald-400">✓ Workspace ready</div>

                  <br />

                  <div>
                    <span className="text-zinc-500">$</span>{" "}
                    <span className="text-blue-400">pwd</span>
                  </div>

                  <div className="text-zinc-300">/home/learner</div>

                  <br />

                  <div>
                    <span className="text-zinc-500">$</span>{" "}
                    <span className="text-blue-400">mkdir projects</span>
                  </div>

                  <div className="text-emerald-400">✓ Directory created</div>

                  <br />

                  <div>
                    <span className="text-zinc-500">$</span>{" "}
                    <span className="text-blue-400">cd projects</span>
                  </div>

                  <div className="text-yellow-400">+10 XP</div>

                  <div className="text-cyan-400">Lesson Complete!</div>

                  <br />

                  <div>
                    <span className="text-zinc-500">$</span>{" "}
                    <span className="mt-4 animate-pulse text-green-400">█</span>
                  </div>
                </div>
              </TerminalContent>
            </HeroTerminal>
          </div>
        </section>

        <section className="hero__features">
          <div className="hero__card">
            <h3>Interactive Lessons</h3>
            <p>Progress by completing real terminal missions.</p>
          </div>

          <div className="hero__card">
            <h3>Real Commands</h3>
            <p>Practice genuine Linux commands instead of simulations.</p>
          </div>

          <div className="hero__card">
            <h3>Documentation</h3>
            <p>Built-in references whenever you need them.</p>
          </div>
        </section>

        <footer className="hero__footer">

  <div className="hero__footer-links">
    <button
      className="footer-link"
      onClick={() => navigate("/app/docs")}
    >
      Documentation
    </button>

    <button
      className="footer-link"
      onClick={() => navigate(user ? "/app" : "/register")}
    >
      {user ? "Continue Learning" : "Get Started"}
    </button>

    <a
      className="footer-link"
      href="https://github.com/Carbon-Org/carbon"
      target="_blank"
      rel="noreferrer"
    >
      GitHub
    </a>
  </div>

  <div className="hero__footer-bottom">
    <span>© {new Date().getFullYear()} Carbon</span>
    <span>Built for people learning Linux.</span>
  </div>
</footer>
      </div>
    </main>
  );
}

function Page({
  title,
  subtitle,
  children,
}: {
  title: string;
  subtitle: string;
  children: React.ReactNode;
}) {
  return (
    <main className="page">
      <div className="page__content">
        <p className="eyebrow">Carbon Academy</p>

        <h1>{title}</h1>

        <p>{subtitle}</p>

        {children}
      </div>
    </main>
  );
}

function GlossaryLinks() {
  return (
    <p className="glossary-links">
      Glossary: <Link to="/app/glossary/directory">directory</Link> ·{" "}
      <Link to="/app/glossary/path">path</Link>
    </p>
  );
}

function LessonPage() {
  const navigate = useNavigate();
  const { lesson, progress } = useLearning();
  console.log(progress);

  if (!lesson) {
    return (
      <Page
        title="Lessons Complete"
        subtitle="You've completed every lesson currently available."
      >
        <section className="lesson">
          <div className="lesson__card">
            <h2>Next Steps</h2>

            <p>
              Continue experimenting in the interactive terminal or browse the
              documentation to deepen your understanding of Carbon.
            </p>
          </div>

          <button
            className="button button--primary"
            onClick={() => navigate("/app/terminal")}
          >
            Open Interactive Terminal
          </button>
        </section>
      </Page>
    );
  }

  const objectives: string[] = [];

  if (lesson.objective?.concept) {
    objectives.push(lesson.objective.concept);
  }

  if (lesson.objective?.command) {
    objectives.push(`Use the '${lesson.objective.command}' command`);
  }

  if (lesson.mission?.objective?.concept) {
    objectives.push(lesson.mission.objective.concept);
  }

  if (lesson.mission?.objective?.command) {
    objectives.push(`Use the '${lesson.mission.objective.command}' command`);
  }

  return (
    <Page
      title={lesson.title}
      subtitle={
        lesson.story ?? lesson.explanation ?? lesson.mission?.description ?? ""
      }
    >
      <section className="lesson">
      {progress && (
  <div className="lesson__card">
    <h2>Course Progress</h2>

    <div
      style={{
        width: "100%",
        height: 10,
        background: "#2f2f2f",
        borderRadius: 999,
        overflow: "hidden",
        marginTop: 8,
      }}
    >
      <div
        style={{
          width: `${(progress.completed / progress.total) * 100}%`,
          height: "100%",
          background: "#22c55e",
          transition: "width 0.3s ease",
        }}
      />
    </div>

    <p style={{ marginTop: 8 }}>
      {progress.completed} / {progress.total} lessons completed
    </p>
  </div>
)}
        <div className="lesson__card">
          <h2>Mission</h2>

          <p>Complete the following objectives using the Carbon terminal.</p>

          <ul className="lesson__objectives">
            {objectives.map((objective) => (
              <li key={objective}>{objective}</li>
            ))}
          </ul>
        </div>

        {lesson.mission?.hints && lesson.mission.hints.length > 0 && (
          <div className="lesson__card">
            <h2>Hints</h2>

            <ul className="lesson__hints">
              {lesson.mission.hints.map((hint) => (
                <li key={hint}>{hint}</li>
              ))}
            </ul>
          </div>
        )}

        <div className="lesson__card">
          <h2>Related Documentation</h2>

          <ul className="lesson__links">
            <li>Filesystem</li>

            {lesson.objective?.command && <li>{lesson.objective.command}</li>}

            {lesson.mission?.objective?.command && (
              <li>{lesson.mission.objective.command}</li>
            )}
          </ul>
        </div>

        <button
          className="button button--primary lesson__launch"
          onClick={() => navigate("/app/terminal")}
        >
          Open Interactive Terminal
        </button>
      </section>
    </Page>
  );
}

function CodeBlock({
  code,
  terminal = false,
}: {
  code: string;
  terminal?: boolean;
}) {
  const [copied, setCopied] = useState(false);
  const copy = async () => {
    await navigator.clipboard.writeText(code);
    setCopied(true);
    window.setTimeout(() => setCopied(false), 1500);
  };
  return (
    <div className={`code-block ${terminal ? "code-block--terminal" : ""}`}>
      <button onClick={() => void copy()}>{copied ? "Copied" : "Copy"}</button>
      <pre>
        <code>{terminal ? `$ ${code}` : code}</code>
      </pre>
    </div>
  );
}

function DocsLayout({ children }: { children: React.ReactNode }) {
  const [query, setQuery] = useState("");
  const filtered = useMemo(
    () =>
      commandDocs.filter(
        (doc) =>
          doc.name.includes(query.toLowerCase()) ||
          doc.summary.toLowerCase().includes(query.toLowerCase()),
      ),
    [query],
  );
  return (
    <main className="docs-shell">
      <aside className="docs-nav">
        <Link className="docs-nav__brand" to="/app/docs">
          Carbon Docs
        </Link>
        <input
          aria-label="Search documentation"
          placeholder="Search…"
          value={query}
          onChange={(event) => setQuery(event.target.value)}
        />
        <div className="docs-nav__rule" />
        <p>Getting Started</p>
        <NavLink to="/app">Terminal workspace</NavLink>
        <p>Filesystem</p>
        <NavLink to="/app/glossary/directory">Directories</NavLink>
        <NavLink to="/app/glossary/path">Paths</NavLink>
        <p>Terminal</p>
        {filtered.map((doc) => (
          <NavLink key={doc.name} to={`/app/docs/${doc.name}`}>
            {doc.name}
          </NavLink>
        ))}
        <p>Lessons</p>
        <NavLink to="/app">Current lesson</NavLink>
        <p>Reference</p>
        <NavLink to="/app/docs">Command reference</NavLink>
      </aside>
      <section className="docs-main">{children}</section>
    </main>
  );
}

function DocsPage() {
  return <DocsArticle command={commandDocs[0]?.name ?? ""} />;
}
function DocsArticle({ command }: { command: string }) {
  const doc = getCommandDoc(command) ?? commandDocs[0];
  if (!doc) return null;
  return (
    <DocsLayout>
      <article className="docs-article">
        <header>
          <p className="eyebrow">Command reference</p>
          <h1>{doc.name}</h1>
          <p className="docs-article__summary">{doc.summary}</p>
        </header>
        <section>
          <h2>Concept</h2>
          <p>{doc.purpose}</p>
        </section>
        <section>
          <h2>Syntax</h2>
          {doc.syntax.map((syntax) => (
            <CodeBlock key={syntax} code={syntax} />
          ))}
        </section>
        {doc.parameters.length > 0 && (
          <section>
            <h2>Arguments</h2>
            <table>
              <thead>
                <tr>
                  <th>Name</th>
                  <th>Required</th>
                  <th>Description</th>
                </tr>
              </thead>
              <tbody>
                {doc.parameters.map((parameter) => (
                  <tr key={parameter.name}>
                    <td>
                      <code>{parameter.name}</code>
                    </td>
                    <td>{parameter.required ? "Yes" : "No"}</td>
                    <td>{parameter.description}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </section>
        )}
        {doc.examples.length > 0 && (
          <section>
            <h2>Examples</h2>
            {doc.examples.map((example, index) => (
              <div className="docs-example" key={index}>
                {example.description && <p>{example.description}</p>}
                <CodeBlock code={example.command} terminal />
                {example.output && <CodeBlock code={example.output} />}
              </div>
            ))}
          </section>
        )}
        {doc.result && (
          <section>
            <h2>Output</h2>
            <p>{doc.result}</p>
          </section>
        )}
        {doc.notes.length > 0 && (
          <section>
            <h2>Notes</h2>
            <ul>
              {doc.notes.map((note) => (
                <li key={note}>{note}</li>
              ))}
            </ul>
          </section>
        )}
        {doc.commonMistakes.length > 0 && (
          <section>
            <h2>Common mistakes</h2>
            {doc.commonMistakes.map((mistake) => (
              <aside
                className="docs-callout docs-callout--warning"
                key={mistake.mistake}
              >
                <strong>{mistake.mistake}</strong>
                <p>{mistake.explanation}</p>
              </aside>
            ))}
          </section>
        )}
        {doc.errors.length > 0 && (
          <section>
            <h2>Errors</h2>
            {doc.errors.map((error) => (
              <aside className="docs-callout" key={error.name}>
                <strong>{error.name}</strong>
                <p>{error.condition}</p>
              </aside>
            ))}
          </section>
        )}
        {doc.limitations.length > 0 && (
          <section>
            <h2>Limitations</h2>
            <ul>
              {doc.limitations.map((item) => (
                <li key={item}>{item}</li>
              ))}
            </ul>
          </section>
        )}
        <section>
          <h2>Related commands</h2>
          <div className="related-links">
            {doc.related.map((related) => (
              <Link key={related} to={`/app/docs/${related}`}>
                {related}
              </Link>
            ))}
          </div>
          <GlossaryLinks />
        </section>
      </article>
    </DocsLayout>
  );
}

function GlossaryPage({ entry }: { entry: string }) {
  const item = getGlossaryEntry(entry);
  return (
    <DocsLayout>
      <article className="docs-article">
        {item ? (
          <Markdown content={item.markdown} />
        ) : (
          <p>Glossary entry not found.</p>
        )}
      </article>
    </DocsLayout>
  );
}

function Settings() {
  const { activeThemeId, setActiveThemeId } = useTheme();
  return (
    <Page
      title="Settings"
      subtitle="Personalise the workspace without changing the learning experience."
    >
      <section className="card theme-settings">
        <div>
          <p className="eyebrow">Appearance</p>
          <h2>Theme</h2>
          <p>Choose the colour system that feels most comfortable for you.</p>
        </div>
        <div className="theme-options">
          {themes.map((theme) => (
            <label
              className={`theme-option ${activeThemeId === theme.id ? "theme-option--selected" : ""}`}
              key={theme.id}
            >
              <input
                type="radio"
                name="theme"
                value={theme.id}
                checked={activeThemeId === theme.id}
                onChange={() => setActiveThemeId(theme.id)}
              />
              <span className="theme-option__swatches">
                <i style={{ background: theme.colors["--canvas"] }} />
                <i style={{ background: theme.colors["--surface"] }} />
                <i style={{ background: theme.colors["--accent"] }} />
              </span>
              <span>
                <strong>{theme.name}</strong>
                <small>{theme.description}</small>
              </span>
            </label>
          ))}
        </div>
      </section>
    </Page>
  );
}

function AppShell() {
  return (
    <div className="app-shell">
      <Header />

      <main className="app-content">
        <Routes>
          <Route
            index
            element={<Navigate to="learn/first-directory" replace />}
          />
          <Route path="learn/:lesson" element={<LessonPage />} />
          <Route path="terminal" element={<Terminal />} />
          <Route path="docs" element={<DocsPage />} />
          <Route path="docs/:command" element={<DocsRoute />} />
          <Route path="glossary/:entry" element={<GlossaryRoute />} />
          <Route path="settings" element={<Settings />} />
          <Route path="*" element={<Navigate to="/app" replace />} />
        </Routes>
      </main>
    </div>
  );
}
function DocsRoute() {
  const { command = "" } = useParams();
  return <DocsArticle command={command} />;
}
function GlossaryRoute() {
  const { entry = "" } = useParams();
  return <GlossaryPage entry={entry} />;
}
export default function App() {
  return (
    <Routes>
      <Route path="/" element={<HeroPage />} />
      <Route path="/login" element={<AuthPage mode="login" />} />
      <Route path="/register" element={<AuthPage mode="register" />} />
      <Route
        path="/app/*"
        element={
          <Protected>
            <AppShell />
          </Protected>
        }
      />
      <Route path="*" element={<Navigate to="/" replace />} />
    </Routes>
  );
}
