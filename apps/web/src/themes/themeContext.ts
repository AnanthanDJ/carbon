import { createContext, useContext } from 'react'

export type ThemeContextValue = { activeThemeId: string; setActiveThemeId: (id: string) => void }
export const ThemeContext = createContext<ThemeContextValue | null>(null)

export function useTheme() {
  const value = useContext(ThemeContext)
  if (!value) throw new Error('useTheme must be used inside ThemeProvider')
  return value
}
