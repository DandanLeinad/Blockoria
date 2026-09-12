import { invoke } from '@tauri-apps/api/core'
import { useEffect, useState } from 'react'
import type { WorldSummaryDto } from './WorldList'

interface CreateBackupProps {
  folderName: string
  onClose: () => void
  onSuccess?: (world: WorldSummaryDto) => void
}

interface CreateBackupResponseDto {
  backup_path: string
  timestamp: string
  world_folder_name: string
}

type Status = 'idle' | 'loading' | 'success' | 'error' | 'loadingWorld'

function getErrorMessage(error: unknown): string {
  if (typeof error === 'string') return error
  if (error && typeof error === 'object' && 'message' in error) {
    const message = (error as { message?: unknown }).message
    if (typeof message === 'string') return message
  }
  return 'Erro desconhecido'
}

export function CreateBackup({ folderName, onClose, onSuccess }: CreateBackupProps) {
  const [status, setStatus] = useState<Status>('loadingWorld')
  const [world, setWorld] = useState<WorldSummaryDto | null>(null)
  const [errorMessage, setErrorMessage] = useState<string | null>(null)
  const [backupInfo, setBackupInfo] = useState<CreateBackupResponseDto | null>(null)

  // Load world data on mount
  useEffect(() => {
    let cancelled = false
    const loadWorld = async () => {
      try {
        const worlds = await invoke<WorldSummaryDto[]>('cmd_list_worlds')
        const found = worlds.find(w => w.folder_name === folderName)
        if (!cancelled) {
          if (found) {
            setWorld(found)
            setStatus('idle')
          } else {
            setStatus('error')
            setErrorMessage('InvalidWorldPath: World not found')
          }
        }
      } catch (err) {
        if (!cancelled) {
          setStatus('error')
          setErrorMessage(getErrorMessage(err))
        }
      }
    }
    loadWorld()
    return () => { cancelled = true }
  }, [folderName])

  // Handle Escape key
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape' && (status === 'idle' || status === 'loadingWorld')) {
        onClose()
      }
    }
    window.addEventListener('keydown', handleKeyDown)
    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [status, onClose])

  const formatVersion = (v: [number, number, number, number, number]) => v.join('.')

  const getBackupDirDisplay = () => {
    const base = '%USERPROFILE%\\AppData\\Roaming\\Blockoria\\backups'
    const location = world?.account_id || 'Shared'
    const timestamp = new Date().toISOString().replace(/[:.]/g, '-').slice(0, 19)
    return `${base}\\${location}\\${folderName.replace('=', '_')}\\${timestamp}\\`
  }

  const handleCreateBackup = async () => {
    if (!world) return
    setStatus('loading')
    setErrorMessage(null)
    try {
      const result = await invoke<CreateBackupResponseDto>('cmd_create_backup', {
        worldFolderName: world.folder_name,
        accountId: world.account_id,
      })
      setBackupInfo(result)
      setStatus('success')
      onSuccess?.(world)
    } catch (err) {
      setStatus('error')
      const msg = getErrorMessage(err)
      setErrorMessage(msg)
    }
  }

  const handleRetry = () => {
    if (status === 'error' && world) {
      handleCreateBackup()
    } else if (status === 'error' && !world) {
      setStatus('loadingWorld')
      setErrorMessage(null)
      // Reload world
      const loadWorld = async () => {
        try {
          const worlds = await invoke<WorldSummaryDto[]>('cmd_list_worlds')
          const found = worlds.find(w => w.folder_name === folderName)
          if (found) {
            setWorld(found)
            setStatus('idle')
          } else {
            setStatus('error')
            setErrorMessage('InvalidWorldPath: World not found')
          }
        } catch (err) {
          setStatus('error')
          setErrorMessage(getErrorMessage(err))
        }
      }
      loadWorld()
    }
  }

  const getErrorDisplay = (msg: string) => {
    if (msg.includes('InvalidWorldPath') || msg.includes('World not found')) {
      return 'Mundo não encontrado'
    }
    if (msg.includes('Permission denied') || msg.includes('Access denied')) {
      return 'Sem permissão para escrever no diretório de backup'
    }
    if (msg.includes('Disk full') || msg.includes('space') || msg.includes('quota')) {
      return 'Espaço insuficiente em disco'
    }
    return `Erro ao criar backup: ${msg}`
  }

  if (status === 'loadingWorld') {
    return (
      <div className="flex items-center justify-center h-[calc(100vh-200px)]">
        <div className="flex flex-col items-center gap-4">
          <div className="relative">
            <svg className="animate-spin h-10 w-10 text-primary" fill="none" viewBox="0 0 24 24" aria-hidden="true">
              <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
              <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
            </svg>
          </div>
          <p className="text-muted-foreground text-sm">Carregando mundo...</p>
        </div>
      </div>
    )
  }

  if (status === 'success' && backupInfo && world) {
    return (
      <div className="max-w-2xl mx-auto space-y-6" role="alert">
        <div className="flex flex-col items-center text-center gap-4">
          <div className="w-16 h-16 rounded-full bg-success/10 flex items-center justify-center">
            <svg className="w-8 h-8 text-success" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
            </svg>
          </div>
          <div>
            <h2 className="text-2xl font-bold text-foreground">Backup criado com sucesso</h2>
            <p className="text-muted-foreground mt-1">{world.level_name}</p>
          </div>
        </div>
        <div className="bg-card border border-border rounded-xl p-6 space-y-3">
          <div className="flex items-center gap-3">
            <svg className="w-5 h-5 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
            </svg>
            <div>
              <p className="text-sm text-muted-foreground">Pasta do backup</p>
              <p className="font-mono text-sm text-foreground break-all">{backupInfo.backup_path}</p>
            </div>
          </div>
          <div className="flex items-center gap-3">
            <svg className="w-5 h-5 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <div>
              <p className="text-sm text-muted-foreground">Data e hora</p>
              <p className="text-sm text-foreground">{new Date(backupInfo.timestamp).toLocaleString('pt-BR')}</p>
            </div>
          </div>
        </div>
        <div className="flex gap-3 pt-2">
          <button
            onClick={() => onSuccess?.(world)}
            className="flex-1 px-4 py-2.5 bg-primary text-primary-foreground rounded-lg hover:bg-primary/90 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 font-medium"
          >
            Ver backups
          </button>
          <button
            onClick={() => {
              setStatus('idle')
              setBackupInfo(null)
            }}
            className="flex-1 px-4 py-2.5 border border-border bg-background hover:bg-muted rounded-lg transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 font-medium"
          >
            Criar outro
          </button>
        </div>
      </div>
    )
  }

  if (status === 'error') {
    return (
      <div className="max-w-md mx-auto space-y-6" role="alert">
        <div className="flex flex-col items-center text-center gap-4">
          <div className="w-16 h-16 rounded-full bg-destructive/10 flex items-center justify-center">
            <svg className="w-8 h-8 text-destructive" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </div>
          <div>
            <h2 className="text-xl font-bold text-foreground">Erro ao criar backup</h2>
            <p className="text-muted-foreground mt-1">{getErrorDisplay(errorMessage || '')}</p>
          </div>
        </div>
        <div className="flex gap-3">
          <button
            onClick={handleRetry}
            className="flex-1 px-4 py-2.5 bg-primary text-primary-foreground rounded-lg hover:bg-primary/90 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 font-medium"
          >
            Tentar novamente
          </button>
          <button
            onClick={onClose}
            className="flex-1 px-4 py-2.5 border border-border bg-background hover:bg-muted rounded-lg transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 font-medium"
          >
            Cancelar
          </button>
        </div>
      </div>
    )
  }

  if (!world) {
    return (
      <div className="max-w-md mx-auto space-y-6 text-center" role="alert">
        <div className="w-16 h-16 rounded-full bg-destructive/10 flex items-center justify-center mx-auto">
          <svg className="w-8 h-8 text-destructive" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
          </svg>
        </div>
        <div>
          <h2 className="text-xl font-bold text-foreground">Mundo não encontrado</h2>
          <p className="text-muted-foreground mt-1">Este mundo pode ter sido movido ou excluído.</p>
        </div>
        <button
          onClick={onClose}
          className="px-4 py-2.5 bg-primary text-primary-foreground rounded-lg hover:bg-primary/90 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 font-medium"
        >
          Voltar à lista
        </button>
      </div>
    )
  }

  return (
    <div className="max-w-2xl mx-auto space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold text-foreground">Criar Backup</h2>
          <p className="text-muted-foreground mt-1">Novo backup para <span className="text-foreground font-medium">{world.level_name}</span></p>
        </div>
        <button
          onClick={onClose}
          className="p-2 rounded-lg hover:bg-muted transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
          aria-label="Fechar"
        >
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <div className="bg-card border border-border rounded-xl p-6 space-y-4">
        <div className="flex items-center gap-3">
          <svg className="w-5 h-5 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
          </svg>
          <div>
            <p className="text-sm text-muted-foreground">Mundo</p>
            <p className="font-medium text-foreground">{world.level_name}</p>
          </div>
        </div>
        <div className="flex items-center gap-3">
          <svg className="w-5 h-5 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
          </svg>
          <div>
            <p className="text-sm text-muted-foreground">Tipo</p>
            <span className={world.is_shared
              ? 'inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-success/10 text-success border border-success/20'
              : 'inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-primary/10 text-primary border border-primary/20'
            }>
              {world.is_shared ? (
                <>
                  <svg className="w-3 h-3 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
                  </svg>
                  Compartilhado
                </>
              ) : (
                <>
                  <svg className="w-3 h-3 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                  </svg>
                  Conta: {world.account_id?.slice(0, 8)}...
                </>
              )}
            </span>
          </div>
        </div>
        <div className="flex items-center gap-3">
          <svg className="w-5 h-5 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
          </svg>
          <div>
            <p className="text-sm text-muted-foreground">Versão</p>
            <p className="font-mono text-foreground">{formatVersion(world.version)}</p>
          </div>
        </div>
      </div>

      <div className="bg-muted/50 border border-border rounded-xl p-6 space-y-2">
        <div className="flex items-center gap-3">
          <svg className="w-5 h-5 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
          </svg>
          <div>
            <p className="text-sm text-muted-foreground">O backup será salvo em:</p>
            <p className="font-mono text-sm text-foreground break-all">{getBackupDirDisplay()}</p>
          </div>
        </div>
      </div>

      <div className="flex gap-3 pt-2">
        <button
          onClick={onClose}
          disabled={status === 'loading'}
          className="flex-1 px-4 py-2.5 border border-border bg-background hover:bg-muted rounded-lg transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 font-medium disabled:opacity-50 disabled:cursor-not-allowed"
        >
          Cancelar
        </button>
        <button
          onClick={handleCreateBackup}
          disabled={status === 'loading'}
          className="flex-1 px-4 py-2.5 bg-primary text-primary-foreground rounded-lg hover:bg-primary/90 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 font-medium disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {status === 'loading' ? (
            <span className="flex items-center justify-center gap-2">
              <svg className="animate-spin h-5 w-5" fill="none" viewBox="0 0 24 24" aria-hidden="true">
                <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
                <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
              </svg>
              Criando backup...
            </span>
          ) : (
            <span className="flex items-center justify-center gap-2">
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
              </svg>
              Criar Backup
            </span>
          )}
        </button>
      </div>

      {status === 'loading' && (
        <div role="status" aria-live="polite" className="sr-only">
          Criando backup...
        </div>
      )}
    </div>
  )
}
