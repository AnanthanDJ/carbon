import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.tsx'
import { BrowserRouter } from 'react-router-dom'
import { ThemeProvider } from './themes/ThemeProvider.tsx'
import { AuthProvider } from './auth/AuthProvider.tsx'
import { LearningProvider } from './features/lesson/LearningProvider.tsx'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <BrowserRouter>
      <ThemeProvider>
        <AuthProvider><LearningProvider><App /></LearningProvider></AuthProvider>
      </ThemeProvider>
    </BrowserRouter>
  </StrictMode>,
)
