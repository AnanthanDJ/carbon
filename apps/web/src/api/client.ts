const API_BASE_URL = import.meta.env.VITE_API_URL ?? '/api'
const TOKEN_KEY = 'carbon-session-token'

export class ApiError extends Error {
  public readonly status: number

  constructor(message: string, status: number) {
    super(message)
    this.status = status
  }
}

export const tokenStore = {
  get: () => localStorage.getItem(TOKEN_KEY),
  set: (token: string) => localStorage.setItem(TOKEN_KEY, token),
  clear: () => localStorage.removeItem(TOKEN_KEY),
}

async function request<T>(path: string, options: RequestInit = {}, authenticated = false): Promise<T> {
  const headers = new Headers(options.headers)
  if (options.body && !headers.has('Content-Type')) headers.set('Content-Type', 'application/json')
  const token = tokenStore.get()
  if (authenticated && token) headers.set('Authorization', `Bearer ${token}`)

  const response = await fetch(`${API_BASE_URL}${path}`, { ...options, headers })
  if (!response.ok) {
    let message = response.status === 401 ? 'Your session has ended. Please sign in again.' : 'Something went wrong. Please try again.'
    try {
      const body = await response.json() as { message?: string; error?: string }
      message = body.message ?? body.error ?? message
    } catch { /* The API may return an empty error response. */ }
    throw new ApiError(message, response.status)
  }
  return response.json() as Promise<T>
}

export type User = { id: string; username: string }
export type Lesson = {
  id: string; title: string; story?: string; explanation?: string; estimated_minutes?: number | string
  estimated_time?: string; mission?: { title?: string; description?: string; objective?: { concept?: string; command?: string }; hints?: string[] }
  objective?: { concept?: string; command?: string }; hints?: string[]; next?: string | null
}
export type LessonOutcome = { completed: boolean; current: Lesson; next: Lesson | null }
export type TerminalResponse = { stdout: string; stderr?: string; cwd: string; lesson: LessonOutcome | null }

export const api = {
  register: (username: string, password: string) => request<{ success: boolean; token: string }>('/auth/register', { method: 'POST', body: JSON.stringify({ username, password }) }),
  login: (username: string, password: string) => request<{ success: boolean; token: string }>('/auth/login', { method: 'POST', body: JSON.stringify({ username, password }) }),
  me: () => request<User>('/auth/me', {}, true),
  currentLesson: () => request<Lesson>('/lessons/current', {}, true),
  execute: (command: string, cwd: string) => request<TerminalResponse>('/terminal', { method: 'POST', body: JSON.stringify({ command, cwd }) }, true),
}
