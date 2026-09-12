# Spec: Criar Backup (Tela 2 de 3)

## Visão Geral
Tela para criar backup de um mundo selecionado. O usuário escolhe um mundo da lista, confirma a ação, e o app cria um backup versionado com timestamp no diretório configurado.

## Requisitos Funcionais

### RF-01: Acessar tela de criar backup
- **Dado** que o usuário está na lista de mundos (`WorldList`)
- **Quando** clica em um mundo (ou botão "Criar backup" no card)
- **Então** navega para tela `CreateBackup` com o mundo pré-selecionado

### RF-02: Confirmar criação de backup
- **Dado** que a tela `CreateBackup` abre com um mundo selecionado
- **Quando** o usuário clica em "Criar backup"
- **Então** deve chamar `cmd_create_backup(world_folder_name, account_id)`
- **E** exibir loading durante a operação
- **E** ao sucesso: mostrar toast "Backup criado com sucesso" + caminho do backup
- **E** ao erro: mostrar mensagem amigável + botão "Tentar novamente"

### RF-03: Validações antes de criar
- **Dado** que o mundo não existe mais no disco
- **Quando** tenta criar backup
- **Então** erro: "Mundo não encontrado" (não deve travar)

- **Dado** que o diretório de backup não tem permissão de escrita
- **Quando** tenta criar backup
- **Então** erro: "Sem permissão para escrever no diretório de backup"

- **Dado** que o disco está cheio
- **Quando** tenta criar backup
- **Então** erro: "Espaço insuficiente em disco"

### RF-04: Feedback visual durante operação
- Loading spinner + "Criando backup..." (disable botão)
- Progresso opcional se backup for grande (futuro)
- Toast de sucesso com timestamp e localização

### RF-05: Navegação pós-sucesso
- **Dado** backup criado com sucesso
- **Quando** usuário clica "Ver backups" (ou similar)
- **Então** navega para tela `ListBackups` filtrada por esse mundo

### RF-06: Acessibilidade
- Botão "Criar backup" com `aria-label` descritivo
- Loading com `aria-live="polite"`
- Toast com `role="alert"`
- Navegação por teclado funcional

## Contrato IPC (já implementado no backend)

```typescript
// Comando Tauri
interface CreateBackupResponseDto {
  backup_path: string;
  timestamp: string;        // ISO 8601
  world_folder_name: string;
}

// invoke('cmd_create_backup', { world_folder_name: string, account_id: string | null })
// -> Promise<CreateBackupResponseDto>
```

## UI/UX

### Layout (Modal ou Página)
```
┌─────────────────────────────────────┐
│ ← Voltar    Criar Backup            │
├─────────────────────────────────────┤
│                                     │
│   Mundo: Mundo Sobrevivência        │
│   Tipo: Compartilhado               │
│   Versão: 1.21.120.0                │
│                                     │
│   ┌─────────────────────────────┐   │
│   │ O backup será salvo em:     │   │
│   │ %APPDATA%\Blockoria\backups\|   │
│   │ \Mundo Sobrevivência\       │   │
│   │ 2026-09-11T14-30-00Z/       │   │
│   └─────────────────────────────┘   │
│                                     │
│   [Cancelar]    [Criar Backup]      │
│                                     │
└─────────────────────────────────────┘
```

### Estados
1. **Inicial** — mostra info do mundo + destino do backup
2. **Loading** — spinner, botão desabilitado
3. **Sucesso** — toast verde + botão "Ver backups" + "Criar outro"
4. **Erro** — toast vermelho + mensagem + "Tentar novamente"

## Critérios de Aceite
1. Da lista de mundos, clicar em mundo → abre CreateBackup pré-preenchido
2. Criar backup → loading → sucesso com toast informativo
3. Erro de I/O → toast com mensagem clara + retry funciona
4. Mundo inexistente → erro "Mundo não encontrado"
5. Navegação por teclado + leitores de tela funcionam
6. `bun test` + `cargo test` passam

## Fora de Escopo
- Agendamento automático de backup
- Compressão/encriptação de backup
- Backup incremental/diferencial
- Notificação push
