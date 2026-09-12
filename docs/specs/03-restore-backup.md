# Spec: Restaurar Backup (Tela 3 de 3)

## Visão Geral
Tela para restaurar um backup previamente criado. O usuário seleciona um backup da lista, confirma a restauração, e o app substitui os arquivos do mundo original pelo backup.

## Requisitos Funcionais

### RF-01: Acessar tela de restaurar backup
- **Dado** que o usuário está na lista de mundos (`WorldList`)
- **Quando** clica em "Ver backups" (ou similar) em um mundo
- **Então** navega para tela `ListBackups` filtrada por esse mundo

### RF-02: Listar backups de um mundo
- **Dado** que a tela `ListBackups` abre para um mundo específico
- **Quando** carrega
- **Então** deve chamar `cmd_list_backups(world_folder_name, account_id)`
- **E** exibir lista de backups ordenados por data (mais recente primeiro)
- **E** cada backup mostra: timestamp, caminho, versão do mundo

### RF-03: Restaurar backup selecionado
- **Dado** que a lista de backups está visível
- **Quando** o usuário clica em "Restaurar" em um backup
- **Então** deve exibir confirmação modal ("Isso substituirá o mundo atual. Continuar?")
- **E** ao confirmar, chama `cmd_restore_backup(backup_path, world_folder_name, account_id)`
- **E** exibir loading durante a operação
- **E** ao sucesso: toast "Backup restaurado com sucesso" + opção voltar à lista de mundos
- **E** ao erro: toast com mensagem + botão "Tentar novamente"

### RF-04: Validações antes de restaurar
- **Dado** que o backup não existe mais no disco
- **Quando** tenta restaurar
- **Então** erro: "Backup não encontrado"

- **Dado** que o mundo original não existe mais
- **Quando** tenta restaurar
- **Então** erro: "Mundo original não encontrado"

- **Dado** que não tem permissão de escrita no mundo
- **Quando** tenta restaurar
- **Então** erro: "Sem permissão para restaurar no diretório do mundo"

### RF-05: Estados de loading e feedback
- Loading spinner + "Restaurando backup..." (disable botões)
- Toast sucesso/erro com `role="alert"`
- Confirmação modal acessível (focus trap, Escape fecha)

### RF-06: Acessibilidade
- Lista navegável por teclado
- Botão "Restaurar" com `aria-label` descritivo
- Modal de confirmação com `role="dialog"`, `aria-modal="true"`
- Toast com `role="alert"`, `aria-live="assertive"`

## Contrato IPC (já implementado no backend)

```typescript
// Listar backups
interface BackupSummaryDto {
  backup_path: string;
  timestamp: string;        // ISO 8601
  world_folder_name: string;
  world_version: [number, number, number, number, number];
}

// invoke('cmd_list_backups', { world_folder_name: string, account_id: string | null })
// -> Promise<BackupSummaryDto[]>

// Restaurar backup
interface RestoreBackupResponseDto {
  success: boolean;
  message: string;
}

// invoke('cmd_restore_backup', {
//   backup_path: string,
//   world_folder_name: string,
//   account_id: string | null
// })
// -> Promise<RestoreBackupResponseDto>
```

## UI/UX

### Layout - Lista de Backups
```
┌─────────────────────────────────────┐
│ ← Voltar    Backups: Mundo X        │
├─────────────────────────────────────┤
│                                     │
│ ┌─────────────────────────────────┐ │
│ │ 2026-09-11 14:30:00             │ │  card por backup
│ │ Versão: 1.21.120.0              │ │
│ │ [Restaurar]                     │ │
│ └─────────────────────────────────┘ │
│                                     │
│ ┌─────────────────────────────────┐ │
│ │ 2026-09-10 08:15:00             │ │
│ │ Versão: 1.21.110.2              │ │
│ │ [Restaurar]                     │ │
│ └─────────────────────────────────┘ │
│                                     │
└─────────────────────────────────────┘
```

### Modal de Confirmação
```
┌─────────────────────────────────────┐
│ ⚠️  Confirmar Restauração           │
├─────────────────────────────────────┤
│                                     │
│ Isso substituirá TODOS os arquivos  │
│ do mundo "Mundo Sobrevivência"      │
│ pelo backup de 2026-09-11 14:30:00. │
│                                     │
│ Esta ação NÃO PODE ser desfeita.    │
│                                     │
│        [Cancelar]  [Restaurar]      │
│                                     │
└─────────────────────────────────────┘
```

## Critérios de Aceite
1. Da lista de mundos, acessar backups de um mundo → lista carregada
2. Backups ordenados por data (mais recente primeiro)
3. Clicar restaurar → modal confirmação → confirmar → loading → sucesso
4. Erro de I/O → toast com mensagem clara + retry
5. Cancelar modal → volta à lista sem restaurar
6. Navegação por teclado + leitores de tela funcionam
7. `bun test` + `cargo test` passam

## Fora de Escopo
- Preview do backup antes de restaurar
- Restauração parcial (selecionar arquivos)
- Backup automático antes de restaurar (snapshot)
- Agendamento de restauração
