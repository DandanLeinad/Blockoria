import { invoke } from '@tauri-apps/api/core'
import { useCallback, useEffect, useRef, useState } from 'react'
import { toast } from 'sonner'
import { Icon } from '../components/ui/Icons'
import { useTheme } from '../components/useTheme'
import { Button } from '../components/ui/button'
import { Card, CardContent } from '../components/ui/card'
import { Label } from '../components/ui/label'

interface ConfigDto {
  theme: 'light' | 'dark' | 'system'
  auto_backup: boolean
}

export function SettingsPage() {
  const { setTheme } = useTheme()
  const [config, setConfig] = useState<ConfigDto>({
    theme: 'system',
    auto_backup: false,
  })
  const [loading, setLoading] = useState(true)
  const [saving, setSaving] = useState(false)
  const loadedRef = useRef(false)

  const loadConfig = useCallback(async (): Promise<ConfigDto | null> => {
    if (loadedRef.current) return null
    loadedRef.current = true
    setLoading(true)
    try {
      const data = await invoke<ConfigDto>('cmd_get_config')
      setConfig(data)
      return data
    } catch {
      toast.error('Erro ao carregar configurações')
      return null
    } finally {
      setLoading(false)
    }
  }, [])

  const reloadConfig = useCallback(async () => {
    loadedRef.current = false
    const savedConfig = await loadConfig()
    if (savedConfig) setTheme(savedConfig.theme)
  }, [loadConfig])

  useEffect(() => {
    // eslint-disable-next-line react-hooks/set-state-in-effect
    loadConfig()
  }, [loadConfig])

  const saveConfig = async () => {
    setSaving(true)
    try {
      await invoke('cmd_save_config', { config })
      setTheme(config.theme)
      toast.success('Configurações salvas com sucesso!')
      loadConfig()
    } catch {
      toast.error('Erro ao salvar configurações')
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

      <Card>
        <CardContent className="space-y-6 p-6">
          <div className="border-t border-border pt-6 space-y-4">
            <h3 className="text-lg font-semibold text-foreground">Aparência</h3>
            <div className="space-y-3">
              <Label className="block text-sm font-medium text-foreground">Tema</Label>
              <div className="grid grid-cols-3 gap-3 sm:grid-cols-3">
                {(['light', 'dark', 'system'] as const).map((theme) => (
                  <Button
                    key={theme}
                    type="button"
                    variant={config.theme === theme ? 'default' : 'outline'}
                    className="p-3 h-auto w-full text-center min-w-0"
                    onClick={() => {
                      setConfig(prev => ({ ...prev, theme }))
                    }}
                  >
                    <div className="font-medium capitalize">{theme === 'system' ? 'Sistema' : theme === 'light' ? 'Claro' : 'Escuro'}</div>
                  </Button>
                ))}
              </div>
            </div>
          </div>

          <div className="flex justify-end gap-3">
            <Button variant="outline" onClick={reloadConfig} disabled={saving}>
              Descartar alterações
            </Button>
            <Button onClick={saveConfig} disabled={saving}>
              {saving ? (
                <span className="flex items-center gap-2">
                  <Icon name="loader" className="animate-spin h-4 w-4" />
                  Salvando...
                </span>
              ) : (
                'Salvar configurações'
              )}
            </Button>
          </div>
      </CardContent>
      </Card>
  </div>
  )
}
