import { useState } from 'react'
import { NavLink, Navigate, Route, Routes, useNavigate } from 'react-router-dom'
import { chapters, lesson } from './data/mockData'
import { Terminal } from './features/terminal/Terminal'
import { themes } from './themes'
import { useTheme } from './themes/themeContext'
import './features/settings/themeSettings.css'
import './App.css'

type IconName = 'home' | 'lesson' | 'terminal' | 'docs'
const navigation: { label: string; path: string; icon: IconName }[] = [
  { label: 'Home', path: '/', icon: 'home' }, { label: 'Lesson', path: '/learn/first-directory', icon: 'lesson' },
  { label: 'Terminal', path: '/terminal', icon: 'terminal' }, { label: 'Docs', path: '/docs', icon: 'docs' },
]
const icons: Record<IconName, string> = { home: '⌂', lesson: '▣', terminal: '>_', docs: '▤' }

function Header() {
  const [menuOpen, setMenuOpen] = useState(false); const [profileOpen, setProfileOpen] = useState(false)
  return <header className="header">
    <button className="menu-button" aria-label="Open navigation" onClick={() => setMenuOpen(true)}>☰</button>
    <NavLink className="brand" to="/terminal"><span className="brand__mark">C</span><span>carbon</span></NavLink>
    <nav className={`nav ${menuOpen ? 'nav--open' : ''}`} aria-label="Main navigation">
      <button className="nav__close" onClick={() => setMenuOpen(false)} aria-label="Close navigation">×</button>
      {navigation.map((item) => <NavLink key={item.path} to={item.path} onClick={() => setMenuOpen(false)} className={({ isActive }) => `nav__link ${isActive ? 'nav__link--active' : ''}`}><span>{icons[item.icon]}</span>{item.label}</NavLink>)}
      <span className="nav__future">Notes <em>soon</em></span>
    </nav>
    {menuOpen && <button className="backdrop" aria-label="Close navigation" onClick={() => setMenuOpen(false)} />}
    <div className="profile"><button className="profile__trigger" onClick={() => setProfileOpen((open) => !open)} aria-expanded={profileOpen}><span className="avatar">AK</span><span className="profile__name">Ananthu</span><span>⌄</span></button>
      {profileOpen && <div className="profile__menu"><NavLink to="/profile" onClick={() => setProfileOpen(false)}>Profile</NavLink><NavLink to="/settings" onClick={() => setProfileOpen(false)}>Settings</NavLink><NavLink to="/about" onClick={() => setProfileOpen(false)}>About</NavLink><button>Log out</button></div>}
    </div>
  </header>
}

function HomePage() { return <Page title="Your learning path" subtitle="A focused way to build command-line confidence."><div className="stats"><Stat value="340" label="XP earned" /><Stat value="2 / 5" label="Lessons complete" /><Stat value="4 days" label="Current streak" /></div><div className="chapter-list">{chapters.map((chapter, index) => <article className="chapter" key={chapter.title}><div><p className="eyebrow">{index === 0 ? 'In progress' : 'Locked'}</p><h2>{chapter.title}</h2></div><strong>{chapter.progress}%</strong><div className="progress"><span style={{ width: `${chapter.progress}%` }} /></div><ul>{chapter.lessons.map((item, lessonIndex) => <li key={item}><span>{index === 0 && lessonIndex < 2 ? '✓' : '⌁'}</span>{item}</li>)}</ul></article>)}</div></Page> }
function LessonPage() { const navigate = useNavigate(); return <Page title={lesson.title} subtitle={lesson.description}><div className="lesson-grid"><article className="card"><p className="eyebrow">Mission objectives</p><h2>What you’ll learn</h2><ol>{lesson.objectives.map((objective) => <li key={objective}>{objective}</li>)}</ol></article><aside className="card lesson-score"><p className="eyebrow">Reward</p><strong>{lesson.score} XP</strong><div className="progress"><span style={{ width: `${lesson.progress}%` }} /></div><p>{lesson.progress}% lesson progress</p><button className="button" onClick={() => navigate('/terminal')}>Open terminal</button></aside></div></Page> }
function DocsPage() { return <div className="docs"><aside className="docs__sidebar"><p className="eyebrow">Command reference</p><a className="docs__selected" href="#mkdir">mkdir</a><a href="#pwd">pwd</a><a href="#ls">ls</a></aside><article className="docs__article"><p className="eyebrow">Filesystem</p><h1 id="mkdir">mkdir</h1><p>Creates a new directory at the path you provide. A directory is a container for files and other directories.</p><h2>Usage</h2><pre><code>mkdir [directory-name]</code></pre><h2>Example</h2><pre><code><span>$ </span>mkdir hello</code></pre><table><thead><tr><th>Argument</th><th>Meaning</th></tr></thead><tbody><tr><td>directory-name</td><td>The name of the new directory.</td></tr></tbody></table></article></div> }
function Page({ title, subtitle, children }: { title: string; subtitle: string; children: React.ReactNode }) { return <main className="page"><div className="page__intro"><p className="eyebrow">Carbon academy</p><h1>{title}</h1><p>{subtitle}</p></div>{children}</main> }
function Stat({ value, label }: { value: string; label: string }) { return <div className="stat"><strong>{value}</strong><span>{label}</span></div> }
function SettingsPage() {
  const { activeThemeId, setActiveThemeId } = useTheme()
  return <Page title="Settings" subtitle="Personalise the workspace without changing the learning experience."><section className="card theme-settings"><div><p className="eyebrow">Appearance</p><h2>Theme</h2><p>Choose the colour system that feels most comfortable for you.</p></div><div className="theme-options">{themes.map((theme) => <label className={`theme-option ${activeThemeId === theme.id ? 'theme-option--selected' : ''}`} key={theme.id}><input type="radio" name="theme" value={theme.id} checked={activeThemeId === theme.id} onChange={() => setActiveThemeId(theme.id)} /><span className="theme-option__swatches"><i style={{ background: theme.colors['--canvas'] }} /><i style={{ background: theme.colors['--surface'] }} /><i style={{ background: theme.colors['--accent'] }} /></span><span><strong>{theme.name}</strong><small>{theme.description}</small></span></label>)}</div></section></Page>
}
function SimplePage({ title }: { title: string }) { return <Page title={title} subtitle="This area is ready to connect to your account API."><div className="card">Your information will appear here when the backend integration is available.</div></Page> }

export default function App() { return <div className="app"><Header /><Routes><Route path="/" element={<HomePage />} /><Route path="/terminal" element={<main className="terminal-page"><Terminal /></main>} /><Route path="/learn/:lesson" element={<LessonPage />} /><Route path="/docs" element={<DocsPage />} /><Route path="/profile" element={<SimplePage title="Your profile" />} /><Route path="/settings" element={<SettingsPage />} /><Route path="/about" element={<SimplePage title="About Carbon" />} /><Route path="*" element={<Navigate to="/terminal" replace />} /></Routes></div> }
