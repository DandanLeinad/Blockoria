import { useState, useEffect, useCallback, useRef } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { Icon } from '../components/ui/Icons'
import { useToast } from '../components/useToast'
import { useTheme } from '../components/useTheme'

interface ConfigDto {
  theme: 'light' | 'dark' | 'system'
  auto_backup: boolean
}

export function SettingsPage() {
  const { success, error: toastError } = useToast()
  const { setTheme } = useTheme()
  const [config, setConfig] = useState<ConfigDto>({
    theme: 'system',
    auto_backup: false,
  })
  const [loading, setLoading] = useState(true)
  const [saving, setSaving] = useState(false)
  const loadedRef = useRef(false)

  const loadConfig = useCallback(async () => {
    if (loadedRef.current) return
    loadedRef.current = true
    setLoading(true)
    try {
      const data = await invoke<ConfigDto>('cmd_get_config')
      setConfig(data)
    } catch {
      toastError('Erro ao carregar configurações')
    } finally {
      setLoading(false)
    }
  }, [toastError])

  const reloadConfig = useCallback(async () => {
    loadedRef.current = false
    await loadConfig()
  }, [loadConfig, toastError])

  useEffect(() => {
    // eslint-disable-next-line react-hooks/set-state-in-effect
    loadConfig()
  }, [loadConfig])

  const saveConfig = async () => {
    setSaving(true)
    try {
      await invoke('cmd_save_config', { config })
      success('Configurações salvas com sucesso!')
      loadConfig()
    } catch {
      toastError('Erro ao salvar configurações')
    } finally {
      setSaving(false)
    }
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center h-[calc(100vh-200px)]">
        <div className="flex flex-col items-center gap-4">
          <Icon name="loader" className="animate-spin h-10 w-10 text-primary" />
          <p className="text-muted-foreground text-sm">Carregando configurações...</p>
        </div>
      </div>
    )
  }

  return (
    <div className="max-w-2xl mx-auto space-y-6">
      <div>
        <h2 className="text-2xl font-bold text-foreground">Configurações</h2>
        <p className="text-muted-foreground mt-1">Gerencie preferências do aplicativo</p>
      </div>

      <div className="bg-card border border-border rounded-xl p-6 space-y-6">
        <div className="border-t border-border pt-6 space-y-4">
          <h3 className="text-lg font-semibold text-foreground">Aparência</h3>
          <div className="space-y-3">
            <label className="block text-sm font-medium text-foreground">Tema</label>
            <div className="grid grid-cols-3 gap-3">
              {(['light', 'dark', 'system'] as const).map((theme) => (
                <button
                  key={theme}
                  type="button"
                  onClick={() => {
                    setConfig(prev => ({ ...prev, theme }))
                    setTheme(theme)
                  }}
                  className={`p-4 rounded-lg border-2 transition-colors text-center ${
                    config.theme === theme
                      ? 'border-primary bg-primary/10 text-primary-foreground'
                      : 'border-border hover:border-primary/50 hover:bg-muted'
                  }`}
                >
                  <div className="text-sm font-medium capitalize">{theme === 'system' ? 'Sistema' : theme === 'light' ? 'Claro' : 'Escuro'}</div>
                  <div className="text-xs text-muted-foreground mt-1">
                    {theme === 'system' ? 'Segue preferência do Windows' : theme === 'light' ? 'Sempre claro' : 'Sempre escuro'}
                  </div>
                </button>
              ))}
            </div>
          </div>
        </div>

        <div className="flex justify-end gap-3">
        <button
          onClick={reloadConfig}
          disabled={saving}
          className="px-4 py-2 border border-border bg-background hover:bg-muted rounded-lg transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
        >
          Descartar alterações
        </button>
        <button
          onClick={saveConfig}
          disabled={saving}
          className="px-4 py-2 bg-primary text-primary-foreground rounded-lg hover:bg-primary/80 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50"
        >
          {saving ? (
            <span className="flex items-center gap-2">
              <Icon name="loader" className="animate-spin h-4 w-4" />
              Salvando...
            </span>
          ) : (
            'Salvar configurações'
          )}
        </button>
      </div>
    </div>
  </div>
  )
}
