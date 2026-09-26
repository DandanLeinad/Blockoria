import { invoke } from '@tauri-apps/api/core'
import { useCallback, useEffect, useRef, useState } from 'react'
import { formatVersion } from '../utils/format'
import { Icon } from './ui/Icons'

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
  const hasLoadedRef = useRef(false)

  const loadWorlds = useCallback(async () => {
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
  }, [])

  useEffect(() => {
    if (!propsWorlds && !hasLoadedRef.current) {
      hasLoadedRef.current = true
      loadWorlds()
    }
  }, [propsWorlds, loadWorlds])

  useEffect(() => {
    if (propsWorlds) {
      // eslint-disable-next-line react-hooks/set-state-in-effect
      setWorlds(propsWorlds)
    }
  }, [propsWorlds])

  const truncateAccountId = (id: string) =>
    id.length > 8 ? `${id.slice(0, 8)}...` : id

  if (loading) {
    return (
      <div className="flex items-center justify-center h-[calc(100vh-200px)]">
        <div className="flex flex-col items-center gap-4">
          <div className="relative">
            <Icon name="loader" className="animate-spin h-10 w-10 text-primary" />
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
            <Icon name="alertTriangle" className="w-8 h-8 text-destructive" />
        </div>
        <div>
          <h3 className="text-lg font-semibold text-foreground">Erro ao carregar mundos</h3>
          <p className="text-muted-foreground text-sm mt-1">{error}</p>
        </div>
        <button
          onClick={loadWorlds}
          className="px-4 py-2 bg-primary text-primary-foreground rounded-lg hover:bg-primary/80 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
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
            <Icon name="folderOpen" className="w-10 h-10 text-muted-foreground" />
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
          className="px-4 py-2 bg-primary text-primary-foreground rounded-lg hover:bg-primary/80 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 mt-2"
        >
          Atualizar
        </button>
      </div>
    )
  }

  return (
    <div className="space-y-5">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-end sm:justify-between">
        <div>
          <p className="text-sm font-medium text-primary">Minecraft Bedrock</p>
          <h2 className="text-2xl font-bold text-foreground">Mundos</h2>
          <p className="text-sm text-muted-foreground mt-1">
            Selecione um mundo para criar um backup ou consultar os existentes.
          </p>
        </div>
        <button
          type="button"
          onClick={loadWorlds}
          disabled={loading}
          className="inline-flex items-center justify-center gap-2 px-3 py-2 border border-border bg-card text-foreground rounded-lg hover:bg-muted transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 font-medium disabled:opacity-50 disabled:cursor-not-allowed"
        >
          <Icon name="refreshCw" className={`w-4 h-4 ${loading ? 'animate-spin' : ''}`} />
          Atualizar
        </button>
      </div>

      <div className="grid grid-cols-1 gap-4" role="list" aria-label="Lista de mundos do Minecraft Bedrock">
        {worlds.map((world) => (
        <article
          key={world.folder_name}
          role="listitem"
          className="group bg-card border border-border rounded-xl p-4 hover:shadow-lg hover:border-primary/50 transition-all duration-200"
        >
          <div className="flex items-start gap-3 sm:gap-4">
            <div className="relative flex-shrink-0">
              {world.icon_path ? (
                <>
                  <img
                    src={world.icon_path}
                    alt={`Ícone do mundo ${world.level_name}`}
                    className="w-24 h-24 rounded-xl object-cover"
                    onError={(e) => {
                      e.currentTarget.classList.add('hidden')
                      e.currentTarget.nextElementSibling?.classList.remove('hidden')
                    }}
                  />
                  <div className="hidden w-24 h-24 rounded-xl bg-gradient-to-br from-primary/10 to-primary/20 items-center justify-center">
                    <Icon name="folderOpen" className="w-8 h-8 text-primary" />
                  </div>
                </>
              ) : (
                <div className="w-24 h-24 rounded-xl bg-gradient-to-br from-primary/10 to-primary/20 flex items-center justify-center">
                  <Icon name="folderOpen" className="w-8 h-8 text-primary" />
                </div>
              )}
            </div>
            <div className="flex-1 min-w-0">
              <div className="flex flex-col gap-2 sm:flex-row sm:items-start sm:justify-between">
                <div>
                  <h3 className="font-semibold text-foreground truncate">{world.level_name}</h3>
                  <p className="text-sm text-muted-foreground font-mono">{world.folder_name}</p>
                </div>
                <div className="flex items-center gap-1.5 flex-shrink-0 self-start">
                  {world.is_shared ? (
                    <span className="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium bg-success/10 text-success border border-success/20">
                      <Icon name="users" className="w-3 h-3 mr-1" />
                      Compartilhado
                    </span>
                  ) : (
                    <span className="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium bg-primary/10 text-primary border border-primary/20">
                      <Icon name="user" className="w-3 h-3 mr-1" />
                      Conta: {truncateAccountId(world.account_id || 'desconhecida')}
                    </span>
                  )}
                </div>
              </div>
              <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:gap-4 mt-3 pt-3 border-t border-border">
                <div className="flex items-center gap-2 text-sm text-muted-foreground">
                  <Icon name="fileText" className="w-4 h-4" />
                  <span>Versão: <code className="font-mono text-foreground">{formatVersion(world.version)}</code></span>
                </div>
                <div className="flex flex-col sm:flex-row gap-2 sm:ml-auto">
                  {onWorldSelect && (
                    <button
                      type="button"
                      onClick={() => onWorldSelect(world)}
                      className="inline-flex items-center justify-center gap-1.5 text-sm font-medium bg-primary text-primary-foreground hover:bg-primary/80 transition-colors px-3 py-1.5 rounded-lg focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
                    >
                      <Icon name="plus" className="w-4 h-4" />
                      Criar backup
                    </button>
                  )}
                  {onViewBackups && (
                  <button
                    type="button"
                    onClick={() => onViewBackups(world)}
                    className="inline-flex items-center justify-center gap-1.5 text-sm font-medium text-primary hover:text-primary/80 transition-colors px-3 py-1.5 rounded-lg hover:bg-primary/10 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
                    aria-label={`Ver backups de ${world.level_name}`}
                  >
                    <Icon name="folderOpen" className="w-4 h-4" />
                    Ver backups
                  </button>
                  )}
                </div>
              </div>
            </div>
          </div>
        </article>
        ))}
      </div>
    </div>
  )
}
