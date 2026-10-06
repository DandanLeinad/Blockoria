import { render, screen, act, waitFor } from '@testing-library/react'
import { describe, it, expect, beforeEach, vi } from 'vitest'
import { ToastProvider } from './ToastContainer'
import { useToast } from './useToast'

describe('ToastContext', () => {
  beforeEach(() => {
    const TestComponent = () => {
      const { toast, success, error, info, warning, dismiss, dismissAll } = useToast()
      return (
        <div>
          <button onClick={() => toast('custom', { type: 'info' })}>Custom</button>
          <button onClick={() => success('success msg')}>Success</button>
          <button onClick={() => error('error msg')}>Error</button>
          <button onClick={() => info('info msg')}>Info</button>
          <button onClick={() => warning('warning msg')}>Warning</button>
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

  it('should provide toast functions via context', () => {
    expect(screen.getByText('Custom')).toBeInTheDocument()
    expect(screen.getByText('Success')).toBeInTheDocument()
    expect(screen.getByText('Error')).toBeInTheDocument()
    expect(screen.getByText('Info')).toBeInTheDocument()
    expect(screen.getByText('Warning')).toBeInTheDocument()
    expect(screen.getByText('Dismiss')).toBeInTheDocument()
    expect(screen.getByText('DismissAll')).toBeInTheDocument()
  })

  it('should throw error when useToast used outside provider', () => {
    const consoleError = vi.spyOn(console, 'error').mockImplementation(() => {})
    const OutsideComponent = () => {
      useToast()
      return null
    }

    expect(() => render(<OutsideComponent />)).toThrow('useToast must be used within a ToastProvider')
    consoleError.mockRestore()
  })

  it('should create toast with correct type via success', async () => {
    await act(async () => {
      screen.getByText('Success').click()
    })

    await waitFor(() => {
      expect(screen.getByRole('status')).toHaveTextContent('success msg')
    })
  })

  it('should create toast with correct type via error', async () => {
    await act(async () => {
      screen.getByText('Error').click()
    })

    await waitFor(() => {
      expect(screen.getByRole('alert')).toHaveTextContent('error msg')
    })
  })

  it('should create toast with correct type via info', async () => {
    await act(async () => {
      screen.getByText('Info').click()
    })

    await waitFor(() => {
      expect(screen.getByRole('status')).toHaveTextContent('info msg')
    })
  })

  it('should create toast with correct type via warning', async () => {
    await act(async () => {
      screen.getByText('Warning').click()
    })

    await waitFor(() => {
      expect(screen.getByRole('status')).toHaveTextContent('warning msg')
    })
  })

  it('should create toast with custom options', async () => {
    await act(async () => {
      screen.getByText('Custom').click()
    })

    await waitFor(() => {
      expect(screen.getByRole('status')).toHaveTextContent('custom')
    })
  })

  it('should dismiss specific toast by id', async () => {
    await act(async () => {
      screen.getByText('Success').click()
    })
    await waitFor(() => {
      expect(screen.getByRole('status')).toBeInTheDocument()
    })

    await act(async () => {
      screen.getByText('Dismiss').click()
    })

    await waitFor(() => {
      expect(screen.queryByRole('status')).not.toBeInTheDocument()
    })
  })

  it('should dismiss all toasts', async () => {
    await act(async () => {
      screen.getByText('Success').click()
      screen.getByText('Error').click()
    })
    await waitFor(() => {
      expect(screen.getAllByRole('status')).toHaveLength(1)
      expect(screen.getAllByRole('alert')).toHaveLength(1)
    })

    await act(async () => {
      screen.getByText('DismissAll').click()
    })

    await waitFor(() => {
      expect(screen.queryByRole('status')).not.toBeInTheDocument()
      expect(screen.queryByRole('alert')).not.toBeInTheDocument()
    })
  })
})
