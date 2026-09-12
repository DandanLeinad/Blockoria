import { invoke } from '@tauri-apps/api/core'
import { useEffect, useState } from 'react'

interface BackupSummaryDto {
  backup_path: string
  timestamp: string
  world_folder_name: string
  world_version: [number, number, number, number, number]
}

interface ListBackupsProps {
  worldFolderName: string
  accountId: string | null
  onClose: () => void
}

interface RestoreBackupResponseDto {
  success: boolean
  message: string
}

interface DeleteBackupResponseDto {
  success: boolean
  message: string
}

function getErrorMessage(error: unknown): string {
  if (typeof error === 'string') return error
  if (error && typeof error === 'object' && 'message' in error) {
    const message = (error as { message?: unknown }).message
    if (typeof message === 'string') return message
  }
  return 'Erro desconhecido'
}

export function ListBackups({ worldFolderName, accountId, onClose }: ListBackupsProps) {
  const [backups, setBackups] = useState<BackupSummaryDto[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [restoreModal, setRestoreModal] = useState<BackupSummaryDto | null>(null)
  const [deleteModal, setDeleteModal] = useState<BackupSummaryDto | null>(null)
  const [restoring, setRestoring] = useState<string | null>(null)
  const [deleting, setDeleting] = useState<string | null>(null)

  const loadBackups = async () => {
    setLoading(true)
    setError(null)
    try {
      const data = await invoke<BackupSummaryDto[]>('cmd_list_backups', {
        worldFolderName,
        accountId,
      })
      data.sort((a, b) => new Date(b.timestamp).getTime() - new Date(a.timestamp).getTime())
      setBackups(data)
    } catch (err) {
      setError(getErrorMessage(err))
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    loadBackups()
  }, [worldFolderName, accountId])

  const formatDate = (iso: string) => {
    const date = new Date(iso)
    return date.toLocaleString('pt-BR', {
      day: '2-digit',
      month: '2-digit',
      year: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    })
  }

  const formatVersion = (v: [number, number, number, number, number]) => v.join('.')

  const handleRestore = async (backup: BackupSummaryDto) => {
    setRestoring(backup.backup_path)
    try {
      await invoke<RestoreBackupResponseDto>('cmd_restore_backup', {
        backupPath: backup.backup_path,
        worldFolderName: backup.world_folder_name,
        accountId,
      })
      setRestoreModal(null)
      alert('Backup restaurado com sucesso!')
      onClose()
    } catch (err) {
      const msg = getErrorMessage(err)
      alert(`Erro ao restaurar: ${msg}`)
    } finally {
      setRestoring(null)
    }
  }

  const handleDelete = async (backup: BackupSummaryDto) => {
    setDeleting(backup.backup_path)
    try {
      await invoke<DeleteBackupResponseDto>('cmd_delete_backup', {
        backupPath: backup.backup_path,
      })
      setBackups(prev => prev.filter(b => b.backup_path !== backup.backup_path))
      setDeleteModal(null)
      alert('Backup deletado com sucesso!')
    } catch (err) {
      const msg = getErrorMessage(err)
      alert(`Erro ao deletar: ${msg}`)
    } finally {
      setDeleting(null)
    }
  }

  const openRestoreModal = (backup: BackupSummaryDto) => {
    setRestoreModal(backup)
  }

  const openDeleteModal = (backup: BackupSummaryDto) => {
    setDeleteModal(backup)
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center h-[calc(100vh-200px)]">
        <div className="flex flex-col items-center gap-4">
          <div className="relative">
            <svg className="animate-spin h-10 w-10 text-primary" fill="none" viewBox="0 0 24 24" aria-hidden="true">
              <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
              <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
            </svg>
          </div>
          <p className="text-muted-foreground text-sm">Carregando backups...</p>
        </div>
      </div>
    )
  }

  if (error) {
    return (
      <div className="flex flex-col items-center justify-center h-[calc(100vh-200px)] gap-4 text-center px-6">
        <div className="w-16 h-16 rounded-full bg-destructive/10 flex items-center justify-center">
          <svg className="w-8 h-8 text-destructive" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
          </svg>
        </div>
        <div>
          <h3 className="text-lg font-semibold text-foreground">Erro ao carregar backups</h3>
          <p className="text-muted-foreground text-sm mt-1">{error}</p>
        </div>
        <button
          onClick={loadBackups}
          className="px-4 py-2 bg-primary text-primary-foreground rounded-lg hover:bg-primary/90 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
        >
          Tentar novamente
        </button>
      </div>
    )
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold text-foreground">Backups</h2>
          <p className="text-muted-foreground mt-1">{worldFolderName}</p>
        </div>
        <button
          onClick={onClose}
          className="px-4 py-2 border border-border bg-background hover:bg-muted rounded-lg transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 font-medium"
        >
          ← Voltar
        </button>
      </div>

      {backups.length === 0 ? (
        <div className="flex flex-col items-center justify-center h-[calc(100vh-250px)] gap-4 text-center px-6">
          <div className="w-20 h-20 rounded-full bg-muted flex items-center justify-center">
            <svg className="w-10 h-10 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M9 21v-6m0 0v-6m0 6H7m4 0h4" />
            </svg>
          </div>
          <div>
            <h3 className="text-lg font-semibold text-foreground">Nenhum backup encontrado</h3>
            <p className="text-muted-foreground text-sm mt-1">Crie o primeiro backup para este mundo na tela principal.</p>
          </div>
        </div>
      ) : (
        <div className="space-y-3" role="list" aria-label="Lista de backups">
          {backups.map((backup) => (
            <article
              key={backup.backup_path}
              role="listitem"
              className="bg-card border border-border rounded-xl p-4 hover:shadow-md hover:border-primary/50 transition-all duration-200"
            >
              <div className="flex items-center justify-between gap-4">
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-3">
                    <div className="w-12 h-12 rounded-lg bg-primary/10 flex items-center justify-center flex-shrink-0">
                      <svg className="w-6 h-6 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M9 21v-6m0 0v-6m0 6H7m4 0h4" />
                      </svg>
                    </div>
                    <div>
                      <p className="font-medium text-foreground">{formatDate(backup.timestamp)}</p>
                      <p className="text-sm text-muted-foreground font-mono">{backup.backup_path}</p>
                    </div>
                  </div>
                  <div className="flex items-center gap-2 mt-2 text-sm text-muted-foreground">
                    <span className="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium bg-muted border border-border">
                      <svg className="w-3 h-3 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                      </svg>
                      Versão: <code className="font-mono text-foreground">{formatVersion(backup.world_version)}</code>
                    </span>
                  </div>
                </div>
                <div className="flex items-center gap-2 flex-shrink-0">
                  <button
                      onClick={() => openRestoreModal(backup)}
                      disabled={restoring === backup.backup_path}
                      className="px-3 py-1.5 bg-primary text-primary-foreground text-sm rounded-lg hover:bg-primary/90 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                      {restoring === backup.backup_path ? (
                        <span className="flex items-center gap-1.5">
                          <svg className="animate-spin h-4 w-4" fill="none" viewBox="0 0 24 24" aria-hidden="true">
                            <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
                            <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
                          </svg>
                          Restaurando...
                        </span>
                      ) : (
                        <span className="flex items-center gap-1.5">
                          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                          </svg>
                          Restaurar
                        </span>
                      )}
                  </button>
                  <button
                      onClick={() => openDeleteModal(backup)}
                      disabled={deleting === backup.backup_path}
                      className="p-1.5 text-red-600 hover:bg-red-50 dark:hover:bg-red-900/20 rounded-lg transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-red-500 focus-visible:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed"
                      aria-label={`Deletar backup de ${formatDate(backup.timestamp)}`}
                    >
                      <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                      </svg>
                  </button>
                </div>
              </div>
            </article>
          ))}
        </div>
      )}

      {/* Modal Restaurar */}
      {restoreModal && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4" role="dialog" aria-modal="true" aria-labelledby="restore-modal-title">
          <div className="bg-card rounded-xl p-6 max-w-md w-full shadow-xl">
            <div className="flex items-center gap-3 mb-4">
              <div className="w-10 h-10 rounded-full bg-yellow-100 dark:bg-yellow-900/30 flex items-center justify-center">
                <svg className="w-5 h-5 text-yellow-600 dark:text-yellow-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                </svg>
              </div>
              <h3 id="restore-modal-title" className="text-lg font-semibold text-foreground">Confirmar Restauração</h3>
            </div>
            <p className="text-muted-foreground mb-4">
              Isso substituirá <strong>TODOS os arquivos</strong> do mundo <strong className="text-foreground">{worldFolderName}</strong>
              pelo backup de <strong className="text-foreground">{formatDate(restoreModal.timestamp)}</strong>.
            </p>
            <p className="text-sm text-destructive mb-6">
              Esta ação <strong>NÃO PODE</strong> ser desfeita.
            </p>
            <div className="flex gap-3 justify-end">
              <button
                onClick={() => setRestoreModal(null)}
                disabled={restoring === restoreModal.backup_path}
                className="px-4 py-2 border border-border bg-background hover:bg-muted rounded-lg transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 font-medium disabled:opacity-50"
              >
                Cancelar
              </button>
              <button
                onClick={() => handleRestore(restoreModal)}
                disabled={restoring === restoreModal.backup_path}
                className="px-4 py-2 bg-destructive text-destructive-foreground rounded-lg hover:bg-destructive/90 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-destructive focus-visible:ring-offset-2 font-medium disabled:opacity-50"
              >
                {restoring === restoreModal.backup_path ? 'Restaurando...' : 'Restaurar'}
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Modal Deletar */}
      {deleteModal && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4" role="dialog" aria-modal="true" aria-labelledby="delete-modal-title">
          <div className="bg-card rounded-xl p-6 max-w-md w-full shadow-xl">
            <div className="flex items-center gap-3 mb-4">
              <div className="w-10 h-10 rounded-full bg-red-100 dark:bg-red-900/30 flex items-center justify-center">
                <svg className="w-5 h-5 text-red-600 dark:text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                </svg>
              </div>
              <h3 id="delete-modal-title" className="text-lg font-semibold text-foreground">Confirmar Exclusão</h3>
            </div>
            <p className="text-muted-foreground mb-4">
              Tem certeza que deseja deletar o backup de <strong className="text-foreground">{formatDate(deleteModal.timestamp)}</strong>?
            </p>
            <p className="text-sm text-destructive mb-6">
              Esta ação <strong>NÃO PODE</strong> ser desfeita.
            </p>
            <div className="flex gap-3 justify-end">
              <button
                onClick={() => setDeleteModal(null)}
                disabled={deleting === deleteModal.backup_path}
                className="px-4 py-2 border border-border bg-background hover:bg-muted rounded-lg transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 font-medium disabled:opacity-50"
              >
                Cancelar
              </button>
              <button
                onClick={() => handleDelete(deleteModal)}
                disabled={deleting === deleteModal.backup_path}
                className="px-4 py-2 bg-destructive text-destructive-foreground rounded-lg hover:bg-destructive/90 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-destructive focus-visible:ring-offset-2 font-medium disabled:opacity-50"
              >
                {deleting === deleteModal.backup_path ? 'Deletando...' : 'Deletar'}
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}
