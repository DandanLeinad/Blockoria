import { createContext, type ReactNode } from 'react'

export type ToastType = 'success' | 'error' | 'info' | 'warning'

export type ToastPosition =
  | 'top-right'
  | 'top-left'
  | 'bottom-right'
  | 'bottom-left'
  | 'top-center'
  | 'bottom-center'

export interface ToastAction {
  label: string
  onClick: () => void
  variant?: 'primary' | 'secondary' | 'ghost'
}

export interface ToastOptions {
  type?: ToastType
  duration?: number
  action?: ToastAction
  richContent?: ReactNode
  persist?: boolean
  position?: ToastPosition
  onClose?: () => void
}

export interface Toast {
  id: number
  message: string
  type: ToastType
  options: ToastOptions
  createdAt: number
}

interface ToastContextValue {
  toast: (message: string, options?: ToastOptions) => number
  success: (message: string, options?: Omit<ToastOptions, 'type'>) => number
  error: (message: string, options?: Omit<ToastOptions, 'type'>) => number
  info: (message: string, options?: Omit<ToastOptions, 'type'>) => number
  warning: (message: string, options?: Omit<ToastOptions, 'type'>) => number
  dismiss: (id: number) => void
  dismissAll: () => void
  toasts: Toast[]
  position: ToastPosition
  maxVisible: number
}

const ToastContext = createContext<ToastContextValue | null>(null)

export { ToastContext, type ToastContextValue }
