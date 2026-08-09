/* eslint-disable react-refresh/only-export-components */
import { createContext, useCallback, useContext, useEffect, useState } from 'react'
import { api, ApiError, tokenStore, type User } from '../api/client'

type AuthContextValue = {
  user: User | null; ready: boolean
  authenticate: (username: string, password: string, mode: 'login' | 'register') => Promise<void>
  logout: () => void
}
const AuthContext = createContext<AuthContextValue | null>(null)

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [user, setUser] = useState<User | null>(null)
  const [ready, setReady] = useState(() => !tokenStore.get())
  const logout = useCallback(() => { tokenStore.clear(); setUser(null) }, [])

  useEffect(() => {
    if (!tokenStore.get()) return
    api.me().then(setUser).catch((error: unknown) => { if (error instanceof ApiError && error.status === 401) logout() }).finally(() => setReady(true))
  }, [logout])

  const authenticate = useCallback(async (username: string, password: string, mode: 'login' | 'register') => {
    const result = mode === 'login' ? await api.login(username, password) : await api.register(username, password)
    tokenStore.set(result.token)
    try { setUser(await api.me()) } catch (error) { logout(); throw error }
  }, [logout])

  return <AuthContext.Provider value={{ user, ready, authenticate, logout }}>{children}</AuthContext.Provider>
}

export function useAuth() {
  const context = useContext(AuthContext)
  if (!context) throw new Error('useAuth must be used inside AuthProvider')
  return context
}
