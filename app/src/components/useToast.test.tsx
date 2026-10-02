import { render, screen, act, waitFor } from '@testing-library/react'
import { describe, it, expect, beforeEach, vi } from 'vitest'
import { useToast } from './useToast'
import { ToastProvider } from './ToastContainer'

describe('useToast hook', () => {
  beforeEach(() => {
    const TestComponent = () => {
      const { toast, success, error, info, warning, dismiss, dismissAll } = useToast()
      return (
        <div>
          <button onClick={() => toast('test')}>Toast</button>
          <button onClick={() => success('success')}>Success</button>
          <button onClick={() => error('error')}>Error</button>
          <button onClick={() => info('info')}>Info</button>
          <button onClick={() => warning('warning')}>Warning</button>
          <button onClick={() => dismiss(1)}>Dismiss</button>
          <button onClick={() => dismissAll()}>DismissAll</button>
        </div>
      )
    }

    render(
      <ToastProvider>
        <TestComponent />
      </ToastProvider>
    )
  })

  it('should return all toast functions', () => {
    expect(screen.getByText('Toast')).toBeInTheDocument()
    expect(screen.getByText('Success')).toBeInTheDocument()
    expect(screen.getByText('Error')).toBeInTheDocument()
    expect(screen.getByText('Info')).toBeInTheDocument()
    expect(screen.getByText('Warning')).toBeInTheDocument()
    expect(screen.getByText('Dismiss')).toBeInTheDocument()
    expect(screen.getByText('DismissAll')).toBeInTheDocument()
  })

  it('should create toast and return id', async () => {
    await act(async () => {
      screen.getByText('Toast').click()
    })

    await waitFor(() => {
      expect(screen.getByRole('status')).toBeInTheDocument()
    })
  })

  it('should throw error when used outside ToastProvider', () => {
    const consoleError = vi.spyOn(console, 'error').mockImplementation(() => {})
    const OutsideComponent = () => {
      useToast()
      return null
    }

    expect(() => render(<OutsideComponent />)).toThrow('useToast must be used within a ToastProvider')
    consoleError.mockRestore()
  })

  it('should provide dismiss and dismissAll functions', async () => {
    await act(async () => {
      screen.getByText('Success').click()
    })
    await waitFor(() => {
      expect(screen.getByRole('status')).toBeInTheDocument()
    })

    await act(async () => {
      screen.getByText('DismissAll').click()
    })

    await waitFor(() => {
      expect(screen.queryByRole('status')).not.toBeInTheDocument()
    })
  })
})
