import { useEffect, useMemo, useState } from 'react'
import { defaultThemeId, themes } from './index'
import { ThemeContext } from './themeContext'

const storageKey = 'carbon-theme'
function getInitialTheme() {
  const stored = localStorage.getItem(storageKey)
  return stored && themes.some((theme) => theme.id === stored) ? stored : defaultThemeId
}

/** Applies the chosen theme's semantic tokens and remembers it across visits. */
export function ThemeProvider({ children }: { children: React.ReactNode }) {
  const [activeThemeId, setActiveThemeId] = useState(getInitialTheme)
  const activeTheme = useMemo(() => themes.find((theme) => theme.id === activeThemeId) ?? themes[0], [activeThemeId])

  useEffect(() => {
    Object.entries(activeTheme.colors).forEach(([property, value]) => document.documentElement.style.setProperty(property, value))
    document.documentElement.dataset.theme = activeTheme.id
    localStorage.setItem(storageKey, activeTheme.id)
  }, [activeTheme])

  return <ThemeContext.Provider value={{ activeThemeId, setActiveThemeId }}>{children}</ThemeContext.Provider>
}
