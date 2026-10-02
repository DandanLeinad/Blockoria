import { useEffect, useState, type ReactNode } from 'react'
import { ThemeContext } from './ThemeContext'

export function ThemeProvider({ children }: { children: ReactNode }) {
  const [theme, setTheme] = useState<'light' | 'dark' | 'system'>(() => {
    if (typeof window !== 'undefined') {
      return (localStorage.getItem('theme') as 'light' | 'dark' | 'system') || 'system'
    }
    return 'system'
  })
  const [resolvedTheme, setResolvedTheme] = useState<'light' | 'dark'>('light')

  const applyTheme = (t: 'light' | 'dark' | 'system') => {
    let resolved: 'light' | 'dark'
    if (t === 'system') {
      resolved = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
    } else {
      resolved = t
    }
    setResolvedTheme(resolved)
    const root = document.documentElement
    root.classList.remove('light', 'dark')
    root.classList.add(resolved)
  }

  // Initial setup - runs once on mount
  useEffect(() => {
    // eslint-disable-next-line react-hooks/set-state-in-effect
    applyTheme(theme)
    localStorage.setItem('theme', theme)

    if (theme === 'system') {
      const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
      const handler = () => applyTheme('system')
      mediaQuery.addEventListener('change', handler)
      return () => mediaQuery.removeEventListener('change', handler)
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])

  // Update when theme changes
  useEffect(() => {
    // eslint-disable-next-line react-hooks/set-state-in-effect
    applyTheme(theme)
    localStorage.setItem('theme', theme)
  }, [theme])

  // Listen for theme changes from other tabs/windows (e.g., Settings page)
  useEffect(() => {
    const handleStorageChange = (e: StorageEvent) => {
      if (e.key === 'theme' && e.newValue) {
        setTheme(e.newValue as 'light' | 'dark' | 'system')
      }
    }
    window.addEventListener('storage', handleStorageChange)
    return () => window.removeEventListener('storage', handleStorageChange)
  }, [])

  return (
    <ThemeContext.Provider value={{ theme, resolvedTheme, setTheme }}>
      {children}
    </ThemeContext.Provider>
  )
}
