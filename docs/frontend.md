---
icon: lucide/monitor
---

# Frontend (Tauri + React + TypeScript)

Documentação técnica da interface gráfica do Blockoria.

> **Status**: ✅ Implementado — **4 telas MVP**, **47 testes unitários**, **contrato de erros estruturado** (spec 07), **shadcn/ui**, **Toast v2**, **Settings**.

---

## 🏗️ Arquitetura

O frontend é um **driving adapter** na arquitetura hexagonal:

```text
┌─────────────────────────────────────────────────────────────┐
│                    React + TypeScript                         │
│  ┌─────────────┐  ┌──────────────┐  ┌────────────────────┐  │
│  │ WorldList   │  │ CreateBackup │  │ ListBackups        │  │
│  │ (Tela 1)    │  │ (Tela 2)     │  │ (Tela 3)           │  │
│  └──────┬──────┘  └──────┬───────┘  └─────────┬──────────┘  │
│         │                │                    │             │
│         ▼                ▼                    ▼             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │              React Router v7                        │   │
│  │  - Nested routes (/world/:folderName/...)           │   │
│  │  - errorElement (RouteError captura erros de rota)  │   │
│  │  - useNavigate, useParams                          │   │
│  └──────────────────────┬──────────────────────┘   │
│                         │                                    │
│         invoke()        │         Tauri IPC                  │
└─────────────────────────┼────────────────────────────────────┘
                          ▼
              ┌─────────────────────────┐
              │      Tauri Commands     │
              │  (src-tauri/src/lib.rs) │
              └───────────┬─────────────┘
                          ▼
              ┌─────────────────────────┐
              │    blockoria-application│
              │      (Use Cases)        │
              └─────────────────────────┘
```

8 commands IPC (`cmd_list_worlds`, `cmd_create_backup`, `cmd_list_backups`, `cmd_restore_backup`, `cmd_delete_backup`, `cmd_get_backup_root`, `cmd_get_config`, `cmd_save_config`) — contratos completos na seção [Integração Tauri](#-integração-tauri-ipc).

---

## 📁 Estrutura

```
app/
├── vite.config.ts          # Vite + React + Tailwind v4 (alias @/ via fileURLToPath)
├── vitest.config.ts        # Vitest + jsdom + RTL (mesmo alias)
├── components.json         # shadcn/ui config
└── src/
    ├── main.tsx            # Entry: ErrorBoundary → App
    ├── App.tsx             # ToastProvider → RouterProvider
    ├── router.tsx          # Rotas + errorElement (RouteError)
    ├── index.css           # Tailwind v4 theme (@theme) — fonte da verdade dos tokens
    ├── lib/utils.ts        # cn() = clsx + tailwind-merge
    ├── components/         # WorldList, CreateBackup, ListBackups, useToast,
    │   │                   # ToastContext, ToastContainer, Theme(.tsx/-Context),
    │   │                   # AppLayout, ErrorBoundary, ErrorScreen, RouteError
    │   └── ui/             # shadcn/ui: button, card, dialog, toast, switch,
    │                       # label, separator, Icons (Lucide)
    ├── pages/              # WorldListPage, CreateBackupPage, ListBackupsPage, SettingsPage
    ├── test/setup.ts       # Vitest setup (mocka invoke + plugins Tauri)
    └── utils/
        ├── format.ts       # formatDate, formatVersion
        └── tauriError.ts   # getTauriError + humanMessage (spec 07)
```

---

## 🎨 Design System (Tailwind v4 + shadcn/ui)

- **Tokens semânticos** via `@theme` em `src/index.css`: cores (background/foreground/card/primary/muted/destructive/success/warning), radius, com variante dark. **Fonte da verdade é o arquivo** — não copiamos os valores aqui.
- **Classes utilitárias**: geradas pelo Tailwind v4 a partir do `@theme` (`.bg-*`, `.text-*`, `.border-*`, opacidades semânticas como `.bg-destructive/10`). Consulte a [documentação do Tailwind](https://tailwindcss.com/docs) para a geração.
- **`.bg-overlay`**: backdrop semântico de modal (adapta light/dark automaticamente).
- **Componentes** shadcn/ui (Radix + Tailwind) em `src/components/ui/`: Button (6 variants), Card, Dialog, Toast, Switch, Label, Separator, Icons (wrapper Lucide tree-shaken).
- **`cn()`** (`src/lib/utils.ts`): `twMerge(clsx(inputs))`.

---

## 🔔 Toast Notifications v2

Sistema completo de notificações (`ToastContext.tsx` + `ToastContainer.tsx`): stack FIFO (5 max, gap 8px), animações slide-in/out com progress bar, pause on hover/focus, action buttons, rich content (`ReactNode`), `persist`, 6 posições, acessibilidade completa (`role="status"/"alert"`, `aria-live`, teclado).

```tsx
const { success, error, toast } = useToast()

success('Backup criado com sucesso!')
error('Falha ao criar backup', {
  action: { label: 'Tentar novamente', onClick: handleRetry },
})
toast('Processando...', { richContent: <ProgressBar value={50} />, persist: true })
```

> API completa e critérios: [`docs/specs/06-toast-notifications.md`](specs/06-toast-notifications.md)

---

## 🧭 Roteamento (React Router v7)

```tsx
// src/router.tsx
export const router = createBrowserRouter([
  {
    path: '/',
    element: (
      <ThemeProvider>
        <ToastProvider>
          <AppLayout />
        </ToastProvider>
      </ThemeProvider>
    ),
    errorElement: <RouteError />,   // spec 07, RF-06
    children: [
      { index: true, element: <WorldListPage /> },
      { path: 'world/:folderName/backup/create', element: <CreateBackupPage /> },
      { path: 'world/:folderName/backups', element: <ListBackupsPage /> },
      { path: 'settings', element: <SettingsPage /> },
    ],
  },
])
```

Não usamos loaders — **cada tela carrega seus dados no próprio componente** (`useEffect` + `invoke`), com estados loading/error/empty. Crashes de rota/render são capturados pelo `errorElement`.

**Layout** (`AppLayout.tsx`): sidebar fixa (desktop) + header mobile, `NavLink` com estado ativo, `<Outlet />` no main.

---

## 📱 Telas (MVP)

| Tela | Rota | Arquivos | Command Tauri |
|------|------|----------|---------------|
| WorldList | `/` | `WorldList.tsx` / `WorldListPage.tsx` | `cmd_list_worlds` |
| CreateBackup | `/world/:folderName/backup/create` | `CreateBackup.tsx` / `CreateBackupPage.tsx` | `cmd_list_worlds`, `cmd_get_backup_root`, `cmd_create_backup` |
| ListBackups | `/world/:folderName/backups` | `ListBackups.tsx` / `ListBackupsPage.tsx` | `cmd_list_backups`, `cmd_restore_backup`, `cmd_delete_backup` |
| Settings | `/settings` | `SettingsPage.tsx` + `Theme.tsx` | `cmd_get_config`, `cmd_save_config` |

Comportamentos comuns a todas: estados loading/empty/error com retry, navegação por teclado (`Escape` fecha modal/tela), toasts de sucesso/erro, mensagens de erro via `humanMessage(getTauriError(err))`.

- **WorldList**: grid de cards com ícone, nome, versão, badge (Compartilhado/Conta); click no card → CreateBackup; "Ver backups" → ListBackups. Tela de erro com mensagem mapeada + detalhe bruto (RF-05).
- **CreateBackup**: confirmação com diretório de destino real (via `cmd_get_backup_root`, fallback informativo se o command falhar); loading no botão; sucesso com "Ver backups" / "Criar outro".
- **ListBackups**: cards ordenados por data (mais recente primeiro), modais de confirmação para Restaurar ("substituirá TODOS os arquivos") e Deletar ("NÃO PODE ser desfeita"), "Abrir pasta no Explorer" via plugin-opener.
- **Settings**: seletor de tema Light/Dark/System, persistido em `%APPDATA%\Blockoria\config.toml`, aplicação imediata via `ThemeContext`.

---

## 🔌 Integração Tauri (IPC)

### Contrato de Erros (`CommandError`, spec 07 RF-03)

Todo command que falha **rejeita com um objeto** (não mais string de debug):

```json
{ "code": "not_found", "message": "not found: backup" }
```

| Code | Quando | Mapeia de |
|------|--------|-----------|
| `not_found` | Agregado procurado não existe | `ApplicationError::NotFound` |
| `validation` | Regra de domínio violada | `ApplicationError::Validation` / `DomainError` |
| `storage` | Falha de I/O real (permissão, disco, path) | `ApplicationError::Storage` / `io::Error` |
| `config` | Ler/gravar `config.toml` | `ApplicationError::Config` |
| `unknown` | Fora da taxonomia (ex.: lock envenenado) | `CommandError::unknown()` |

`message` permanece técnica (inglês, para log/debug). A tradução PT-BR é do frontend.

### Extração e Tradução (`src/utils/tauriError.ts`, RF-04)

```typescript
export function getTauriError(error: unknown): TauriError
// Extrai { code, message } de qualquer rejeição do invoke() — NUNCA
// descarta o payload: shapes desconhecidos viram { code: 'unknown',
// message: <payload bruto> }.

export function humanMessage(error: TauriError): string
// Mapa PT-BR por code:
// not_found   → "Mundo ou backup não encontrado"
// validation  → "Dados inválidos"
// storage     → "Erro ao acessar arquivos (permissões ou disco)"
// config      → "Erro nas configurações"
// unknown     → "Erro desconhecido" (+ payload bruto como detalhe)
```

Uso: `humanMessage(getTauriError(err))` — padrão em toasts (`Erro ao restaurar: ...`), banners e telas de erro. **Bugfix RF-05**: o WorldList usava `err instanceof Error` (sempre falso para a string do Tauri) e mostrava "Erro desconhecido" fixo — agora mostra a mensagem mapeada, com o payload bruto como linha de detalhe quando `unknown`.

### Crash de Render (RF-06)

- **`ErrorBoundary`** (classe) envolve o app no `main.tsx` — última linha de defesa para crashes fora do router
- **`RouteError`** é o `errorElement` da rota raiz — o React Router captura erros de rota/render primeiro
- Ambos renderizam o **`ErrorScreen`** compartilhado ("Algo deu errado" + Recarregar); o RouteError inclui o detalhe bruto do erro
- **Tela branca extinta**

### DTOs TypeScript (contrato do frontend)

```typescript
// src/components/WorldList.tsx
export interface WorldSummaryDto {
  folder_name: string
  level_name: string
  version: [number, number, number, number, number]
  is_shared: boolean
  account_id: string | null
  icon_path: string | null
}

// src/components/ListBackups.tsx
interface BackupSummaryDto {
  backup_path: string
  icon_path: string | null
  timestamp: string
  world_folder_name: string
  world_version: [number, number, number, number, number]
}

// src/components/CreateBackup.tsx
interface CreateBackupResponseDto {
  backup_path: string
  timestamp: string
  world_folder_name: string
}

// src-tauri/src/lib.rs (serializado para o frontend)
interface ConfigDto {
  theme: string  // 'light' | 'dark' | 'system'
}
```

### Invocação

```typescript
import { invoke } from '@tauri-apps/api/core'

const worlds = await invoke<WorldSummaryDto[]>('cmd_list_worlds')

const result = await invoke<CreateBackupResponseDto>('cmd_create_backup', {
  worldFolderName: world.folder_name,   // camelCase → snake_case automático
  accountId: world.account_id,
})

await invoke('cmd_save_config', { config: { theme: 'dark' } })
```

> **Atenção**: os argumentos chegam ao Rust em **camelCase** (conversão automática do Tauri para `snake_case`).

---

## 🧪 Testes

Vitest + React Testing Library + jsdom. Setup em `src/test/setup.ts` (mocka `@tauri-apps/api/core` e os plugins). O alias `@/` está configurado no `vitest.config.ts` via `fileURLToPath`.

Padrão obrigatório (AGENTS.md) — blocos Given/When/Then comentados:

```tsx
it('given a structured rejection when loading fails then shows the mapped human message', async () => {
  // Given
  vi.mocked(invoke).mockRejectedValue({ code: 'storage', message: 'permission denied' })

  // When
  render(<WorldList />)

  // Then
  await waitFor(() => {
    expect(screen.getByText('Erro ao carregar mundos')).toBeInTheDocument()
  })
})
```

### Cobertura Unit

| Arquivo | Testes | Cenários |
|---------|--------|----------|
| ToastContainer.test.tsx | 20 | Stack, animações, progress, hover pause, action, persist |
| ToastContext.test.tsx | 9 | Create, dismiss, dismissAll, types, options |
| useToast.test.tsx | 4 | Hook usage, provider check |
| tauriError.test.ts | 10 | Extração `{code, message}`, payload nunca descartado, mapa PT-BR (spec 07, RF-04) |
| WorldList.test.tsx | 2 | Regressão RF-05: mensagem mapeada + payload bruto como detalhe |
| ErrorBoundary.test.tsx | 2 | RF-06: throw de filho → fallback com Recarregar; filho são passa |
| **Total Unit** | **47** | |

```bash
cd app && bun run test         # 47 passed
cd app && bun run build        # TypeScript + Vite build OK
```

---

## ⚙️ Configuração

> **Fonte da verdade são os arquivos** — este doc não copia configs (cópia apodrece):
>
> | Arquivo | O que define |
> |---------|--------------|
> | `vite.config.ts` | Plugins React/Tailwind, alias `@/` (`fileURLToPath`), dev server porta 1420 |
> | `vitest.config.ts` | jsdom, setup, alias `@/`, glob de testes |
> | `tsconfig.app.json` | `verbatimModuleSyntax`, `moduleResolution: bundler`, strict |
> | `components.json` | shadcn/ui (style new-york, baseColor slate, aliases `@/`) |
> | `src-tauri/tauri.conf.json` | `frontendDist: ../app/dist`, `devUrl: http://localhost:1420`, hooks bun |
> | `package.json` | Dependências e scripts |

---

## 🚀 Comandos de Desenvolvimento

```bash
cd app && bun run dev         # Vite dev server (http://localhost:1420)
cargo tauri dev               # Rust + frontend + janela Tauri
cd app && bun run build       # dist/ para Tauri
cargo tauri build             # Bundle completo (MSI/EXE)

# Testes
cd app && bun run test        # Frontend (47 passed)
cargo test                    # Backend (237 passed)
```

---

## 🔮 Próximos Passos (Pós-MVP)

| Item | Prioridade |
|------|------------|
| Toast "promise" (auto-show loading → success/error) | Média |
| Configurações avançadas (diretório de backup, auto-backup) | Média |
| Histórico de toasts (notification center) | Baixa |
| Sons/notificações do sistema | Baixa |
| Drag & drop para restaurar | Baixa |
| Preview do backup antes de restaurar | Baixa |
| Internacionalização (i18n) | Baixa |

---

## 📚 Referências

- [SDD Specs](specs/) — 01–04 telas, 05 NBT, 06 Toast, **07 Tratamento de Erros**
- [BDD Features](../tests/features/) — Cenários Gherkin
- [ADR-0001: Taxonomia de erros](adr/0001-error-taxonomy.md)
- [Domain Docs](domain.md) · [Application Docs](application.md)
- [Tauri v2](https://tauri.app/v2/) · [React Router v7](https://reactrouter.com/) · [Tailwind v4](https://tailwindcss.com/docs) · [shadcn/ui](https://ui.shadcn.com/)
