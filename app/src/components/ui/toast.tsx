'use client'

import { Toaster as Sonner } from 'sonner'
import { cn } from '@/lib/utils'

export function Toaster({ position = 'top-right', richColors = false, ...props }: {
  position?: 'top-left' | 'top-right' | 'bottom-left' | 'bottom-right' | 'top-center' | 'bottom-center'
  richColors?: boolean
}) {
  return (
    <Sonner
      theme="system"
      position={position}
      richColors={richColors}
      className={cn('toaster group')}
      toastOptions={{
        classNames: {
          toast: 'group toast group-[.toaster]:bg-background group-[.toaster]:text-foreground group-[.toaster]:border-border group-[.toaster]:shadow-lg',
          description: 'group-[.toast]:text-muted-foreground',
          actionButton: 'group-[.toast]:bg-primary group-[.toast]:text-primary-foreground',
          cancelButton: 'group-[.toast]:bg-muted group-[.toast]:text-muted-foreground',
        },
      }}
      {...props}
    />
  )
}
