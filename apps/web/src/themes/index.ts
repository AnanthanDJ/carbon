import type { Theme } from './types'

// Vite discovers every theme module in this folder at build time. Adding a new
// `*.theme.ts` file that default-exports Theme makes it available in Settings.
const themeModules = import.meta.glob<Theme>('./*.theme.ts', { eager: true, import: 'default' })

export const themes = Object.values(themeModules).filter((theme) => theme.id !== undefined)
export const defaultThemeId = 'dark'
