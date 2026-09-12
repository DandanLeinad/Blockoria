import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'

export interface WorldSummaryDto {
  folder_name: string
  level_name: string
  version: [number, number, number, number, number]
  is_shared: boolean
  account_id: string | null
  icon_path: string | null
}

interface WorldListProps {
  worlds?: WorldSummaryDto[]
  onWorldSelect?: (world: WorldSummaryDto) => void
  onViewBackups?: (world: WorldSummaryDto) => void
}

export function WorldList({ worlds: propsWorlds, onWorldSelect, onViewBackups }: WorldListProps) {
  const [worlds, setWorlds] = useState<WorldSummaryDto[]>(propsWorlds || [])
  const [loading, setLoading] = useState(!propsWorlds)
  const [error, setError] = useState<string | null>(null)

  const loadWorlds = async () => {
    setLoading(true)
    setError(null)
    try {
      const data = await invoke<WorldSummaryDto[]>('cmd_list_worlds')
      setWorlds(data)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Erro desconhecido')
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    if (!propsWorlds) {
      loadWorlds()
    }
  }, [propsWorlds])

  useEffect(() => {
    if (propsWorlds) {
      setWorlds(propsWorlds)
    }
  }, [propsWorlds])

  const formatVersion = (v: [number, number, number, number, number]) =>
    v.join('.')

  const truncateAccountId = (id: string) =>
    id.length > 8 ? `${id.slice(0, 8)}...` : id

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
          <p className="text-muted-foreground text-sm">Carregando mundos...</p>
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
          <h3 className="text-lg font-semibold text-foreground">Erro ao carregar mundos</h3>
          <p className="text-muted-foreground text-sm mt-1">{error}</p>
        </div>
        <button
          onClick={loadWorlds}
          className="px-4 py-2 bg-primary text-primary-foreground rounded-lg hover:bg-primary/90 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
        >
          Tentar novamente
        </button>
      </div>
    )
  }

  if (worlds.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center h-[calc(100vh-200px)] gap-4 text-center px-6">
        <div className="w-20 h-20 rounded-full bg-muted flex items-center justify-center">
          <svg className="w-10 h-10 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M9 21v-6m0 0v-6m0 6H7m4 0h4" />
          </svg>
        </div>
        <div>
          <h3 className="text-lg font-semibold text-foreground">Nenhum mundo encontrado</h3>
          <p className="text-muted-foreground text-sm mt-1 max-w-md">
            Verifique se o Minecraft Bedrock está instalado e tem mundos criados.
          </p>
        </div>
        <div className="text-xs text-muted-foreground space-y-1 max-w-md px-4">
          <p><code className="bg-muted px-1.5 py-0.5 rounded">{'%APPDATA%\\Minecraft Bedrock\\Users\\<seu_id>\\games\\com.mojang\\minecraftWorlds\\'}</code></p>
          <p>ou na pasta Shared:</p>
          <p><code className="bg-muted px-1.5 py-0.5 rounded">{'%APPDATA%\\Minecraft Bedrock\\Users\\Shared\\games\\com.mojang\\minecraftWorlds\\'}</code></p>
        </div>
        <button
          onClick={loadWorlds}
          className="px-4 py-2 bg-primary text-primary-foreground rounded-lg hover:bg-primary/90 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 mt-2"
        >
          Atualizar
        </button>
      </div>
    )
  }

  return (
    <div className="grid grid-cols-1 gap-4" role="list" aria-label="Lista de mundos do Minecraft Bedrock">
      {worlds.map((world) => (
        <article
          key={world.folder_name}
          role="listitem"
          tabIndex={0}
          onClick={() => onWorldSelect?.(world)}
          onKeyDown={(e) => {
            if (e.key === 'Enter' || e.key === ' ') {
              e.preventDefault()
              onWorldSelect?.(world)
            }
          }}
          className="group bg-card border border-border rounded-xl p-4 hover:shadow-lg hover:border-primary/50 transition-all duration-200 cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
        >
          <div className="flex items-start gap-4">
            <div className="relative flex-shrink-0">
              {world.icon_path ? (
                <img
                  src={world.icon_path}
                  alt={`Ícone do mundo ${world.level_name}`}
                  className="w-16 h-16 rounded-lg object-cover"
                  onError={(e) => {
                    e.currentTarget.style.display = 'none'
                  }}
                />
              ) : (
                <div className="w-16 h-16 rounded-lg bg-gradient-to-br from-primary/10 to-primary/20 flex items-center justify-center">
                  <svg className="w-8 h-8 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M9 21v-6m0 0v-6m0 6H7m4 0h4" />
                  </svg>
                </div>
              )}
            </div>
            <div className="flex-1 min-w-0">
              <div className="flex items-start justify-between gap-2">
                <div>
                  <h3 className="font-semibold text-foreground truncate">{world.level_name}</h3>
                  <p className="text-sm text-muted-foreground font-mono">{world.folder_name}</p>
                </div>
                <div className="flex items-center gap-1.5 flex-shrink-0">
                  {world.is_shared ? (
                    <span className="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium bg-success/10 text-success border border-success/20">
                      <svg className="w-3 h-3 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
                      </svg>
                      Compartilhado
                    </span>
                  ) : (
                    <span className="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium bg-primary/10 text-primary border border-primary/20">
                      <svg className="w-3 h-3 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                      </svg>
                      Conta: {truncateAccountId(world.account_id || 'desconhecida')}
                    </span>
                  )}
                </div>
              </div>
              <div className="flex items-center gap-4 mt-3 pt-3 border-t border-border">
                <div className="flex items-center gap-2 text-sm text-muted-foreground">
                  <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                  </svg>
                  <span>Versão: <code className="font-mono text-foreground">{formatVersion(world.version)}</code></span>
                </div>
                {onViewBackups && (
                  <button
                    onClick={(e) => {
                      e.stopPropagation()
                      onViewBackups(world)
                    }}
                    className="ml-auto inline-flex items-center gap-1.5 text-sm font-medium text-primary hover:text-primary/80 transition-colors px-3 py-1.5 rounded-lg hover:bg-primary/5 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
                    aria-label={`Ver backups de ${world.level_name}`}
                  >
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 21v-6m0 0v-6m0 6H7m4 0h4" />
                    </svg>
                    Ver backups
                  </button>
                )}
              </div>
            </div>
          </div>
        </article>
      ))}
    </div>
  )
}
