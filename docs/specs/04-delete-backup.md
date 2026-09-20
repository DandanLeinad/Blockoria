# Spec: Deletar Backup (Tela 4 de 3 - Operação extra)

## Visão Geral
Operação para deletar um backup específico. Diferente das outras telas, esta é uma ação destrutiva simples que pode ser feita direto na lista de backups (botão por item) ou via modal de confirmação.

## Requisitos Funcionais

### RF-01: Deletar backup da lista
- **Dado** que a lista de backups (`ListBackups`) está visível
- **Quando** o usuário clica no ícone/botão "Deletar" (🗑️) em um backup
- **Então** deve exibir modal de confirmação ("Tem certeza? Esta ação não pode ser desfeita.")
- **E** ao confirmar, chama `cmd_delete_backup(backup_path)`
- **E** exibir loading durante a operação
- **E** ao sucesso: remover item da lista + toast "Backup deletado com sucesso"
- **E** ao erro: toast com mensagem + item permanece na lista

### RF-02: Validações
- **Dado** que o backup não existe mais no disco
- **Quando** tenta deletar
- **Então** erro: "Backup não encontrado" (idempotente - OK se já não existe)

- **Dado** que não tem permissão de exclusão
- **Quando** tenta deletar
- **Então** erro: "Sem permissão para deletar este backup"

### RF-03: Feedback visual
- Botão deletar com ícone 🗑️ + `aria-label="Deletar backup de {timestamp}"`
- Loading no botão durante deleção (disable)
- Toast sucesso/erro
- Lista atualiza automaticamente (remover item)

### RF-04: Acessibilidade
- Botão deletar navegável por teclado
- Modal confirmação com focus trap
- Toast com `role="alert"`

## Contrato IPC (já implementado no backend)

```typescript
// Deletar backup
interface DeleteBackupResponseDto {
  success: boolean;
  message: string;
}

// invoke('cmd_delete_backup', { backup_path: string })
// -> Promise<DeleteBackupResponseDto>
```

## UI/UX

### Na lista de backups (botão por item)
```
┌─────────────────────────────────────┐
│ 2026-09-11 14:30:00    [Restaurar] [🗑️] │
│ Versão: 1.21.120.0                   │
└─────────────────────────────────────┘
```

### Modal de Confirmação
```
┌─────────────────────────────────────┐
│ 🗑️  Confirmar Exclusão             │
├─────────────────────────────────────┤
│                                     │
│ Tem certeza que deseja deletar      │
│ o backup de 2026-09-11 14:30:00?    │
│                                     │
│ Esta ação NÃO PODE ser desfeita.    │
│                                     │
│        [Cancelar]  [Deletar]        │
│                                     │
└─────────────────────────────────────┘
```

## Critérios de Aceite
1. Na lista de backups, botão deletar por item
2. Clicar deletar → modal confirmação → confirmar → loading → item removido + toast
3. Cancelar modal → nada acontece
4. Erro → toast + item permanece
5. Lista vazia após deletar último → estado vazio
6. `bun test` + `cargo test` passam

## Fora de Escopo
- Deletar múltiplos backups de vez (batch)
- Lixeira/recuperação de backup deletado
- Política de retenção automática
