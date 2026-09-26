import { invoke } from '@tauri-apps/api/core'
import { useEffect, useState } from 'react'
import type { WorldSummaryDto } from './WorldList'
import { formatVersion } from '../utils/format'
import { getErrorMessage } from '../utils/error'
import { Icon } from './ui/Icons'

interface CreateBackupProps {
  folderName: string
  levelName?: string
  onClose: () => void
  onSuccess?: (world: WorldSummaryDto) => void
}

interface CreateBackupResponseDto {
  backup_path: string
  timestamp: string
  world_folder_name: string
}

type Status = 'idle' | 'loading' | 'success' | 'error' | 'loadingWorld'

export function CreateBackup({ folderName, levelName, onClose, onSuccess }: CreateBackupProps) {
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
      if ((e.key === 'Escape' || e.key === 'Esc') && (status === 'idle' || status === 'loadingWorld')) {
        onClose()
      }
    }
    window.addEventListener('keydown', handleKeyDown)
    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [status, onClose])

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
            <Icon name="loader" className="animate-spin h-10 w-10 text-primary" />
          </div>
          <p className="text-muted-foreground text-sm">Carregando mundo{folderName ? ` (${levelName ?? folderName})` : ''}...</p>
        </div>
      </div>
    )
  }

  if (status === 'success' && backupInfo && world) {
    return (
      <div className="max-w-2xl mx-auto space-y-6" role="alert">
        <div className="flex flex-col items-center text-center gap-4">
          <div className="w-16 h-16 rounded-full bg-success/10 flex items-center justify-center">
            <Icon name="check" className="w-8 h-8 text-success" />
          </div>
          <div>
            <h2 className="text-2xl font-bold text-foreground">Backup criado com sucesso</h2>
            <p className="text-muted-foreground mt-1">{world.level_name}</p>
          </div>
        </div>
        <div className="bg-card border border-border rounded-xl p-6 space-y-3">
          <div className="flex items-center gap-3">
            <Icon name="folderOpen" className="w-5 h-5 text-muted-foreground" />
            <div>
              <p className="text-sm text-muted-foreground">Pasta do backup</p>
              <p className="font-mono text-sm text-foreground break-all">{backupInfo.backup_path}</p>
            </div>
          </div>
          <div className="flex items-center gap-3">
            <Icon name="clock" className="w-5 h-5 text-muted-foreground" />
            <div>
              <p className="text-sm text-muted-foreground">Data e hora</p>
              <p className="text-sm text-foreground">{new Date(backupInfo.timestamp).toLocaleString('pt-BR')}</p>
            </div>
          </div>
        </div>
        <div className="flex gap-3 pt-2">
          <button
            onClick={() => onSuccess?.(world)}
            className="flex-1 px-4 py-2.5 bg-primary text-primary-foreground rounded-lg hover:bg-primary/80 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 font-medium"
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
            <Icon name="xCircle" className="w-8 h-8 text-destructive" />
          </div>
          <div>
            <h2 className="text-xl font-bold text-foreground">Erro ao criar backup</h2>
            <p className="text-muted-foreground mt-1">{getErrorDisplay(errorMessage || '')}</p>
          </div>
        </div>
        <div className="flex gap-3">
          <button
            onClick={handleRetry}
            className="flex-1 px-4 py-2.5 bg-primary text-primary-foreground rounded-lg hover:bg-primary/80 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 font-medium"
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
            <Icon name="alertTriangle" className="w-8 h-8 text-destructive" />
        </div>
        <div>
          <h2 className="text-xl font-bold text-foreground">Mundo não encontrado</h2>
          <p className="text-muted-foreground mt-1">Este mundo pode ter sido movido ou excluído.</p>
        </div>
        <button
          onClick={onClose}
          className="p-2 border border-border bg-background hover:bg-muted rounded-lg transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
          aria-label="Voltar à lista"
        >
          <Icon name="arrowLeft" className="w-5 h-5" />
        </button>
      </div>
    )
  }

  return (
    <div className="max-w-2xl mx-auto space-y-6">
      <div className="flex items-start justify-between gap-4">
        <div>
          <h2 className="text-2xl font-bold text-foreground">Criar Backup</h2>
          <p className="text-muted-foreground mt-1">Novo backup para <span className="text-foreground font-medium">{world.level_name}</span></p>
        </div>
      </div>

      <div className="bg-card border border-border rounded-xl p-6 space-y-4">
        <div className="flex items-center gap-3">
          <Icon name="folderOpen" className="w-5 h-5 text-muted-foreground" />
          <div>
            <p className="text-sm text-muted-foreground">Mundo</p>
            <p className="font-medium text-foreground">{world.level_name}</p>
          </div>
        </div>
        <div className="flex items-center gap-3">
          <Icon name="user" className="w-5 h-5 text-muted-foreground" />
          <div>
            <p className="text-sm text-muted-foreground">Tipo</p>
            <span className={world.is_shared
              ? 'inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-success/10 text-success border border-success/20'
              : 'inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-primary/10 text-primary border border-primary/20'
            }>
              {world.is_shared ? (
                <>
                  <Icon name="users" className="w-3 h-3 mr-1" />
                  Compartilhado
                </>
              ) : (
                <>
                  <Icon name="user" className="w-3 h-3 mr-1" />
                  Conta: {world.account_id?.slice(0, 8)}...
                </>
              )}
            </span>
          </div>
        </div>
        <div className="flex items-center gap-3">
          <Icon name="fileText" className="w-5 h-5 text-muted-foreground" />
          <div>
            <p className="text-sm text-muted-foreground">Versão</p>
            <p className="font-mono text-foreground">{formatVersion(world.version)}</p>
          </div>
        </div>
      </div>

      <div className="bg-muted/50 border border-border rounded-xl p-6 space-y-2">
        <div className="flex items-center gap-3">
          <Icon name="externalLink" className="w-5 h-5 text-muted-foreground" />
          <div>
            <p className="text-sm text-muted-foreground">O backup será salvo em:</p>
            <p className="font-mono text-sm text-foreground break-all">{getBackupDirDisplay()}</p>
          </div>
        </div>
      </div>

      <div className="flex flex-col sm:flex-row gap-3 pt-2">
        <button
          onClick={onClose}
          disabled={status === 'loading'}
          className="flex-1 p-2.5 border border-border bg-background hover:bg-muted rounded-lg transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed"
          aria-label="Cancelar"
        >
          <Icon name="arrowLeft" className="w-5 h-5 mx-auto" />
        </button>
        <button
          onClick={handleCreateBackup}
          disabled={status === 'loading'}
          className="flex-1 px-4 py-2.5 bg-primary text-primary-foreground rounded-lg hover:bg-primary/80 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 font-medium disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {status === 'loading' ? (
            <span className="flex items-center justify-center gap-2">
              <Icon name="loader" className="animate-spin h-5 w-5" />
              Criando backup...
            </span>
          ) : (
            <span className="flex items-center justify-center gap-2">
              <Icon name="plus" className="w-5 h-5" />
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
