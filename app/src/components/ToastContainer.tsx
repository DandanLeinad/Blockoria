import { useState, useCallback, useEffect, type ReactNode, useContext } from 'react'
import { createPortal } from 'react-dom'
import { ToastContext, type Toast, type ToastOptions, type ToastPosition } from './ToastContext'
import { Icon } from './ui/Icons'

function getPositionClasses(position: ToastPosition): string {
  switch (position) {
    case 'top-right':
      return 'top-4 right-4'
    case 'top-left':
      return 'top-4 left-4'
    case 'bottom-right':
      return 'bottom-4 right-4'
    case 'bottom-left':
      return 'bottom-4 left-4'
    case 'top-center':
      return 'top-4 left-1/2 -translate-x-1/2'
    case 'bottom-center':
      return 'bottom-4 left-1/2 -translate-x-1/2'
    default:
      return 'top-4 right-4'
  }
}

function ToastItem({
  toast,
  onDismiss
}: {
  toast: Toast
  onDismiss: (id: number) => void
}) {
  const { message, type, options, id } = toast
  const { duration = 4000, persist, onClose } = options

  const [isExiting, setIsExiting] = useState(false)
  const [progress, setProgress] = useState(100)
  const [isPaused, setIsPaused] = useState(false)
  const [startTime, setStartTime] = useState<number>(() => Date.now())
  const [pausedTime, setPausedTime] = useState<number>(0)

  const dismiss = useCallback(() => {
    setIsExiting(true)
    setTimeout(() => {
      onDismiss(id)
      onClose?.()
    }, 200)
  }, [id, onDismiss, onClose])

  useEffect(() => {
    if (persist || duration <= 0) return

    const elapsedBeforePause = pausedTime
    const targetDuration = duration
    let animationFrame: number

    const animate = () => {
      if (isPaused || isExiting) {
        animationFrame = requestAnimationFrame(animate)
        return
      }

      const now = Date.now()
      const elapsed = elapsedBeforePause + (now - startTime)
      const newProgress = Math.max(0, 100 - (elapsed / targetDuration) * 100)
      setProgress(newProgress)

      if (newProgress <= 0) {
        dismiss()
        return
      }

      animationFrame = requestAnimationFrame(animate)
    }

    animationFrame = requestAnimationFrame(animate)

    return () => {
      cancelAnimationFrame(animationFrame)
    }
  }, [isPaused, isExiting, persist, duration, startTime, pausedTime, dismiss])

  const handleMouseEnter = () => {
    if (persist || duration <= 0) return
    setIsPaused(true)
    setPausedTime(prev => prev + (Date.now() - startTime))
  }

  const handleMouseLeave = () => {
    if (persist || duration <= 0) return
    setIsPaused(false)
    setStartTime(Date.now())
  }

  const handleFocus = handleMouseEnter
  const handleBlur = handleMouseLeave

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Escape') {
      dismiss()
    }
  }

  if (isExiting) {
    return null
  }

  const typeClasses = {
    success: 'bg-success text-success-foreground border-l-success',
    error: 'bg-destructive text-destructive-foreground border-l-destructive',
    warning: 'bg-warning text-warning-foreground border-l-warning',
    info: 'bg-primary text-primary-foreground border-l-primary'
  }

  const role = type === 'error' ? 'alert' : 'status'
  const live = type === 'error' ? 'assertive' : 'polite'

  return (
    <div
      className={`flex items-start gap-3 px-4 py-3 rounded-lg border-l-4 shadow-lg min-w-[280px] max-w-md animate-slide-in relative overflow-hidden group ${typeClasses[type] || typeClasses.info}`}
      role={role}
      aria-live={live}
      aria-atomic="true"
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      onFocus={handleFocus}
      onBlur={handleBlur}
      onKeyDown={handleKeyDown}
      tabIndex={0}
    >
      {!persist && duration > 0 && (
        <div
          className="absolute top-0 left-0 h-1 bg-current/30 transition-transform duration-[200ms] ease-linear origin-left"
          style={{ transform: `scaleX(${progress / 100})` }}
          aria-hidden="true"
        />
      )}

      <div className="flex items-start gap-3 w-full">
        <Icon
          name={
            type === 'success' ? 'check' :
            type === 'error' ? 'x' :
            type === 'warning' ? 'alertTriangle' :
            'info'
          }
          className="w-5 h-5 flex-shrink-0 mt-0.5"
        />
        <p className="text-sm font-medium flex-1 min-w-0">
          {toast.options.richContent ? toast.options.richContent : message}
        </p>
        <div className="flex items-center gap-2 flex-shrink-0">
          {toast.options.action && (
            <button
              onClick={() => {
                toast.options.action?.onClick()
                dismiss()
              }}
              className="px-2 py-1 text-xs font-medium rounded hover:bg-current/20 transition-colors flex-shrink-0"
            >
              {toast.options.action.label}
            </button>
          )}
          <button
            onClick={dismiss}
            className="flex-shrink-0 p-1 rounded hover:bg-current/20 transition-colors"
            aria-label="Fechar"
          >
            <Icon name="x" className="w-4 h-4" />
          </button>
        </div>
      </div>
    </div>
  )
}

function ToastContainerInternal({ position = 'top-right', maxVisible = 5 }: { position?: string; maxVisible?: number }) {
  const context = useContext(ToastContext)
  if (!context) {
    return null
  }
  const { toasts, dismiss } = context

  const toastElements = toasts.slice(-maxVisible).map((t: typeof toasts[0]) => (
    <ToastItem
      key={t.id}
      toast={t}
      onDismiss={dismiss}
    />
  ))

  const portal = typeof document !== 'undefined'
    ? createPortal(
        <div
          className={`fixed z-50 flex flex-col gap-2 ${getPositionClasses(position as ToastPosition)}`}
          data-testid="toast-container"
        >
          {toastElements}
        </div>,
        document.body
      )
    : null

  return portal
}

export function ToastProvider({
  children,
  maxVisible = 5
}: {
  children: ReactNode
  maxVisible?: number
}) {
  const [toasts, setToasts] = useState<Toast[]>([])
  const [idCounter, setIdCounter] = useState(0)

  const addToast = useCallback((message: string, options: ToastOptions = {}) => {
    const id = idCounter + 1
    setIdCounter(id)
    const newToast = {
      id,
      message,
      type: options.type || 'info',
      options,
      createdAt: Date.now()
    }
    setToasts(prev => [...prev, newToast].slice(-maxVisible))
    return id
  }, [idCounter, maxVisible])

  const dismiss = useCallback((id: number) => {
    setToasts(prev => prev.filter(t => t.id !== id))
  }, [])

  const dismissAll = useCallback(() => {
    setToasts([])
  }, [])

  const contextValue = {
    toast: (message: string, options?: ToastOptions) => addToast(message, options),
    success: (message: string, options?: Omit<ToastOptions, 'type'>) => addToast(message, { ...options, type: 'success' }),
    error: (message: string, options?: Omit<ToastOptions, 'type'>) => addToast(message, { ...options, type: 'error' }),
    info: (message: string, options?: Omit<ToastOptions, 'type'>) => addToast(message, { ...options, type: 'info' }),
    warning: (message: string, options?: Omit<ToastOptions, 'type'>) => addToast(message, { ...options, type: 'warning' }),
    dismiss,
    dismissAll,
    toasts,
    position: 'top-right' as ToastPosition,
    maxVisible: 5
  }

  return (
    <ToastContext.Provider value={contextValue}>
      {children}
      <ToastContainerInternal position="top-right" maxVisible={5} />
    </ToastContext.Provider>
  )
}

export { ToastContext, type ToastContextValue } from './ToastContext'
export { useToast } from './useToast'
export type { Toast, ToastOptions, ToastType, ToastPosition, ToastAction } from './ToastContext'
