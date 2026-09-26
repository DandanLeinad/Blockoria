import { useCallback, useState, type ReactNode } from 'react'
import { createPortal } from 'react-dom'
import { ToastContext, type Toast, type ToastType } from './ToastContext'
import { Icon } from './ui/Icons'

export function ToastProvider({ children }: { children: ReactNode }) {
  const [toasts, setToasts] = useState<Toast[]>([])
  const [idCounter, setIdCounter] = useState(0)

  const addToast = useCallback((message: string, type: ToastType = 'info') => {
    const id = idCounter + 1
    setIdCounter(id)
    setToasts(prev => [...prev, { id, message, type }])
    setTimeout(() => {
      setToasts(prev => prev.filter(t => t.id !== id))
    }, 4000)
  }, [idCounter])

  const removeToast = useCallback((id: number) => {
    setToasts(prev => prev.filter(t => t.id !== id))
  }, [])

  const toast = useCallback((message: string, type: ToastType = 'info') => addToast(message, type), [addToast])
  const success = useCallback((message: string) => addToast(message, 'success'), [addToast])
  const error = useCallback((message: string) => addToast(message, 'error'), [addToast])
  const info = useCallback((message: string) => addToast(message, 'info'), [addToast])

  const toastElements = toasts.map(t => (
    <div
      key={t.id}
      className={`fixed bottom-4 right-4 z-50 flex items-center gap-3 px-4 py-3 rounded-lg shadow-lg min-w-[280px] max-w-md animate-slide-in ${
        t.type === 'success' ? 'bg-success text-success-foreground' :
        t.type === 'error' ? 'bg-destructive text-destructive-foreground' :
        'bg-primary text-primary-foreground'
      }`}
      role="alert"
      aria-live="polite"
    >
      {t.type === 'success' && <Icon name="check" className="w-5 h-5 flex-shrink-0" />}
      {t.type === 'error' && <Icon name="x" className="w-5 h-5 flex-shrink-0" />}
      {t.type === 'info' && <Icon name="info" className="w-5 h-5 flex-shrink-0" />}
      <p className="text-sm font-medium flex-1">{t.message}</p>
      <button
        onClick={() => removeToast(t.id)}
        className="flex-shrink-0 p-1 rounded hover:bg-muted/10 transition-colors"
        aria-label="Fechar"
      >
        <Icon name="x" className="w-4 h-4" />
      </button>
    </div>
  ))

  const portal = typeof document !== 'undefined' ? createPortal(toastElements, document.body) : null

  return (
    <ToastContext.Provider value={{ toast, success, error, info }}>
      {children}
      {portal}
    </ToastContext.Provider>
  )
}
