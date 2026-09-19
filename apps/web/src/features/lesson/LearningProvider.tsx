/* eslint-disable react-refresh/only-export-components */
import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useState,
} from "react";
import {
  api,
  ApiError,
  type CourseProgress,
  type Lesson,
  type LessonOutcome,
} from "../../api/client";
import { useAuth } from "../../auth/AuthProvider";

type LearningContextValue = {
  lesson: Lesson | null;
  outcome: LessonOutcome | null;
  progress: CourseProgress | null;
  cwd: string;
  setCwd: (cwd: string) => void;
  applyOutcome: (outcome: LessonOutcome | null) => void;
  refreshLesson: () => Promise<void>;
};
const LearningContext = createContext<LearningContextValue | null>(null);

export function LearningProvider({ children }: { children: React.ReactNode }) {
  const { user } = useAuth();
  const [lesson, setLesson] = useState<Lesson | null>(null);
  const [outcome, setOutcome] = useState<LessonOutcome | null>(null);
  const [progress, setProgress] = useState<CourseProgress | null>(null);
  const [cwd, setCwd] = useState("/home");
  const refreshLesson = useCallback(async () => {
    if (!user) {
      setLesson(null);
      setProgress(null);
      return;
    }

    try {
      const [lesson, progress] = await Promise.all([
        api.currentLesson(),
        api.progress(),
      ]);

      setLesson(lesson);
      setProgress(progress);
    } catch (error) {
      if (!(error instanceof ApiError && error.status === 404)) throw error;

      setLesson(null);
      setProgress(await api.progress());
    }
  }, [user]);
  useEffect(() => {
    void (async () => {
      try {
        await refreshLesson();
      } catch (error) {
        console.error(error);
      }
    })();
  }, [refreshLesson]);
  const applyOutcome = useCallback((nextOutcome: LessonOutcome | null) => {
    setOutcome(nextOutcome);

    if (!nextOutcome) return;

    setLesson(nextOutcome.next ?? nextOutcome.current);

    if (nextOutcome.completed) {
      setProgress((previous) =>
        previous
          ? {
              ...previous,
              completed: Math.min(previous.completed + 1, previous.total),
            }
          : previous,
      );
    }
  }, []);
  return (
    <LearningContext.Provider
      value={{
        lesson,
        outcome,
        progress,
        cwd,
        setCwd,
        applyOutcome,
        refreshLesson,
      }}
    >
      {children}
    </LearningContext.Provider>
  );
}
export function useLearning() {
  const context = useContext(LearningContext);
  if (!context)
    throw new Error("useLearning must be used inside LearningProvider");
  return context;
}
