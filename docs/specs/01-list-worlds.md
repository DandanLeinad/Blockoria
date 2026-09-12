# Spec: Listar Mundos (Tela 1 de 3)

## Visão Geral
Tela principal do Blockoria. Lista todos os mundos do Minecraft Bedrock Edition detectados nos diretórios padrão (por conta Xbox e Shared), exibindo nome do nível, versão, ícone e se é mundo compartilhado.

## Requisitos Funcionais

### RF-01: Carregar lista de mundos ao abrir
- **Dado** que o app inicia
- **Quando** o componente `WorldList` monta
- **Então** deve chamar `cmd_list_worlds` via IPC
- **E** exibir loading enquanto carrega
- **E** renderizar lista com dados retornados

### RF-02: Exibir dados de cada mundo
Para cada mundo, mostrar:
- `folder_name` (nome da pasta)
- `level_name` (nome do nível no jogo)
- `version` (array [major, minor, patch, build, revision])
- `is_shared` (badge "Compartilhado" ou conta Xbox)
- `account_id` (se não shared)
- `icon_path` (imagem do mundo, se existir)

### RF-03: Estados de erro
- **Dado** que `cmd_list_worlds` falha (I/O, permissão, path inválido)
- **Quando** erro retorna
- **Então** exibir mensagem amigável + botão "Tentar novamente"
- **E** logar erro no console

### RF-04: Lista vazia
- **Dado** que nenhum mundo encontrado
- **Então** exibir estado vazio: "Nenhum mundo encontrado" + ajuda (onde procurar)

### RF-05: Acessibilidade
- Lista navegável por teclado (Tab/Enter)
- `role="list"` + `role="listitem"`
- `aria-label` no container
- Imagens com `alt` descritivo

## Contrato IPC (já implementado no backend)

```typescript
// Comando Tauri
interface WorldSummaryDto {
  folder_name: string;
  level_name: string;
  version: [number, number, number, number, number];
  is_shared: boolean;
  account_id: string | null;
  icon_path: string | null;
}

// invoke('cmd_list_worlds') -> Promise<WorldSummaryDto[]>
```

## UI/UX

### Layout
```
┌─────────────────────────────────────┐
│ Blockoria                           │
├─────────────────────────────────────┤
│ [🔄] Carregando...                  │  (loading)
│                                     │
│ ┌─────────────────────────────────┐ │
│ │ 📁 Meu Mundo Sobrevivência      │ │  card por mundo
│ │ Versão: 1.21.120.0              │ │
│ │ [Compartilhado]                 │ │
│ └─────────────────────────────────┘ │
│                                     │
│ ┌─────────────────────────────────┐ │
│ │ 📁 Creative World               │ │
│ │ Versão: 1.20.80.1               │ │
│ │ Conta: abc123...                │ │
│ └─────────────────────────────────┘ │
└─────────────────────────────────────┘
```

### Componentes sugeridos
- `WorldList` — container, fetch, estados
- `WorldCard` — renderiza um mundo
- `WorldIcon` — imagem com fallback
- `EmptyState` — lista vazia
- `ErrorState` — erro + retry

## Critérios de Aceite
1. App abre → mostra loading → lista mundos reais do `%APPDATA%\Minecraft Bedrock\`
2. Mundos Shared e por conta aparecem corretos
3. Ícone carrega se `icon_path` existir
4. Erro de I/O → toast/alert + retry funciona
5. Lista vazia → estado vazio informativo
6. Navegação por teclado funcional
7. `bun test` + `cargo test` passam

## Fora de Escopo (próximas specs)
- Criar backup (spec 02)
- Restaurar backup (spec 03)
- Deletar backup (spec 04)
- Filtros/ordenacao
- Detalhes do mundo (modal)
