/* eslint-disable react-refresh/only-export-components */
import { createContext, useCallback, useContext, useEffect, useState } from 'react'
import { api, ApiError, type Lesson, type LessonOutcome } from '../../api/client'
import { useAuth } from '../../auth/AuthProvider'

type LearningContextValue = { lesson: Lesson | null; outcome: LessonOutcome | null; cwd: string; setCwd: (cwd: string) => void; applyOutcome: (outcome: LessonOutcome | null) => void; refreshLesson: () => Promise<void> }
const LearningContext = createContext<LearningContextValue | null>(null)

export function LearningProvider({ children }: { children: React.ReactNode }) {
  const { user } = useAuth(); const [lesson, setLesson] = useState<Lesson | null>(null); const [outcome, setOutcome] = useState<LessonOutcome | null>(null); const [cwd, setCwd] = useState('/home')
  const refreshLesson = useCallback(async () => {
    if (!user) { setLesson(null); return }
    try { setLesson(await api.currentLesson()) } catch (error) { if (!(error instanceof ApiError && error.status === 404)) throw error; setLesson(null) }
  }, [user])
  useEffect(() => {
    if (!user) return
    api.currentLesson().then(setLesson).catch((error: unknown) => {
      if (error instanceof ApiError && error.status === 404) setLesson(null)
    })
  }, [user])
  const applyOutcome = useCallback((nextOutcome: LessonOutcome | null) => { setOutcome(nextOutcome); if (nextOutcome) setLesson(nextOutcome.next ?? nextOutcome.current) }, [])
  return <LearningContext.Provider value={{ lesson, outcome, cwd, setCwd, applyOutcome, refreshLesson }}>{children}</LearningContext.Provider>
}
export function useLearning() { const context = useContext(LearningContext); if (!context) throw new Error('useLearning must be used inside LearningProvider'); return context }
