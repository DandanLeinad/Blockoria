---
icon: lucide/monitor
---

# Frontend (Tauri + React + TypeScript)

Documentação técnica da interface gráfica do Blockoria.

> **Status**: ✅ Implementado — **4 telas MVP**, **40 testes unitários**, **6 testes E2E**, integração Tauri completa, **shadcn/ui**, **Toast v2**, **Settings**, **Playwright E2E**.

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
│  │  - Loaders (carregam dados antes de renderizar)     │   │
│  │  - useNavigate, useParams, useLoaderData            │   │
│  └──────────────────────┬──────────────────────────────┘   │
│                         │                                    │
│         invoke()        │         Tauri IPC                  │
└─────────────────────────┼────────────────────────────────────┘
                          ▼
              ┌─────────────────────────┐
              │      Tauri Commands     │
              │  (src-tauri/src/lib.rs) │
              │                         │
              │  cmd_list_worlds        │
              │  cmd_create_backup      │
              │  cmd_list_backups       │
              │  cmd_restore_backup     │
              │  cmd_delete_backup      │
              │  cmd_get_backup_root    │
              │  cmd_get_config         │  ← NOVO
              │  cmd_save_config        │  ← NOVO
              └───────────┬─────────────┘
                          ▼
              ┌─────────────────────────┐
              │    blockoria-application│
              │      (Use Cases)        │
              └─────────────────────────┘
```

---

## 📁 Estrutura de Arquivos

```
app/
├── package.json
├── vite.config.ts          # Vite + React + Tailwind v4
├── vitest.config.ts        # Vitest + jsdom + RTL
├── playwright.config.ts    # Playwright E2E config
├── tsconfig.json           # Project references
├── tsconfig.app.json       # App config (verbatimModuleSyntax)
├── components.json         # shadcn/ui config
├── src/
│   ├── main.tsx                    # Entry point → RouterProvider + ToastProvider
│   ├── App.tsx                     # <RouterProvider router={router} />
│   ├── router.tsx                  # React Router v7 config
│   ├── index.css                   # Tailwind v4 theme (CSS variables)
│   ├── lib/
│   │   └── utils.ts                # cn() = clsx + tailwind-merge
│   ├── components/
│   │   ├── WorldList.tsx           # Tela 1: Lista de mundos
│   │   ├── CreateBackup.tsx        # Tela 2: Criar backup
│   │   ├── ListBackups.tsx         # Tela 3: Listar/restaurar/deletar
│   │   ├── SettingsPage.tsx        # Tela 4: Configurações (NOVA)
│   │   ├── useToast.ts             # Toast hook
│   │   ├── ToastContext.tsx        # Toast context v2
│   │   ├── ToastContainer.tsx      # Toast stack + animations
│   │   ├── Theme.tsx               # Theme provider + toggle
│   │   ├── ThemeContext.tsx        # Theme context
│   │   ├── AppLayout.tsx           # Layout com sidebar/header
│   │   ├── ui/                     # shadcn/ui components
│   │   │   ├── button.tsx
│   │   │   ├── card.tsx
│   │   │   ├── dialog.tsx
│   │   │   ├── toast.tsx
│   │   │   ├── switch.tsx
│   │   │   ├── label.tsx
│   │   │   ├── separator.tsx
│   │   │   └── Icons.tsx
│   ├── pages/
│   │   ├── WorldListPage.tsx       # Página lista de mundos
│   │   ├── CreateBackupPage.tsx    # Página criar backup
│   │   ├── ListBackupsPage.tsx     # Página listar backups
│   │   └── SettingsPage.tsx        # Página configurações (NOVA)
│   ├── test/
│   │   └── setup.ts                # Vitest setup (mocks @tauri-apps/api)
│   └── utils/
│       ├── format.ts               # formatDate, formatVersion
│       └── error.ts                # getErrorMessage
├── e2e/                            # Playwright E2E tests (NOVO)
│   ├── list-worlds.spec.ts
│   ├── create-backup.spec.ts
│   ├── list-backups.spec.ts
│   ├── restore-backup.spec.ts
│   ├── delete-backup.spec.ts
│   └── full-flow.spec.ts
├── public/
└── index.html
```

---

## 🎨 Design System (Tailwind v4 + shadcn/ui)

### Tokens Semânticos (CSS Variables)

Definidos em `src/index.css` via `@theme`:

```css
@theme {
  /* Light mode */
  --color-background: #ffffff;
  --color-foreground: #0f172a;
  --color-card: #ffffff;
  --color-card-foreground: #0f172a;
  --color-primary: #1e3a8a;
  --color-primary-foreground: #ffffff;
  --color-secondary: #f1f5f9;
  --color-secondary-foreground: #0f172a;
  --color-muted: #f1f5f9;
  --color-muted-foreground: #64748b;
  --color-accent: #f1f5f9;
  --color-destructive: #dc2626;
  --color-destructive-foreground: #ffffff;
  --color-border: #e2e8f0;
  --color-input: #e2e8f0;
  --color-ring: #1e3a8a;
  --color-success: #059669;
  --color-warning: #d97706;
  --radius: 0.5rem;
}

/* Dark mode automático via prefers-color-scheme */
@media (prefers-color-scheme: dark) {
  @theme {
    --color-background: #020617;
    --color-foreground: #f8fafc;
    --color-card: #0f172a;
    --color-card-foreground: #f8fafc;
    --color-primary: #3b82f6;
    --color-primary-foreground: #0f172a;
    --color-secondary: #1e293b;
    --color-muted: #1e293b;
    --color-muted-foreground: #94a3b8;
    --color-border: #1e293b;
    --color-input: #1e293b;
    --color-ring: #3b82f6;
    --color-success: #10b981;
    --color-warning: #f59e0b;
  }
}
```

### Classes Utilitárias Disponíveis

```css
/* Backgrounds (Tailwind v4 gera automaticamente do @theme) */
.bg-background, .bg-foreground, .bg-card, .bg-card-foreground
.bg-popover, .bg-popover-foreground
.bg-primary, .bg-primary-foreground
.bg-secondary, .bg-secondary-foreground
.bg-muted, .bg-muted-foreground
.bg-accent, .bg-accent-foreground
.bg-destructive, .bg-destructive-foreground
.bg-success, .bg-success-foreground
.bg-warning, .bg-warning-foreground

/* Opacidades semânticas (geradas automaticamente) */
.bg-primary\/10, .bg-primary\/15, .bg-primary\/20, .bg-primary\/50, .bg-primary\/80
.bg-destructive\/10, .bg-destructive\/15, .bg-destructive\/20
.bg-warning\/10, .bg-warning\/15, .bg-warning\/20
.bg-success\/10, .bg-success\/15, .bg-success\/20
.bg-muted\/10, .bg-muted\/15, .bg-muted\/50

/* Text */
.text-foreground, .text-muted-foreground, .text-primary
.text-primary-foreground, .text-secondary-foreground
.text-destructive, .text-destructive-foreground
.text-success, .text-warning

/* Borders */
.border-border, .border-input, .border-ring
.border-primary, .border-destructive, .border-success, .border-warning

/* Radius */
.rounded, .rounded-lg, .rounded-xl, .rounded-full

/* Shadows (adaptam automaticamente light/dark) */
.shadow-sm, .shadow, .shadow-md, .shadow-lg, .shadow-xl

/* Transitions */
.transition-all, .transition-colors, .transition-shadow

/* Hover/Focus states (padronizados) */
.hover\:bg-primary\/80:hover
.hover\:bg-destructive\/15:hover
.hover\:bg-success\/15:hover
.hover\:bg-warning\/15:hover
.hover\:bg-muted\/50:hover
.hover\:border-primary:hover
.focus-visible\:ring-2:focus-visible
.focus-visible\:ring-ring:focus-visible
.focus-visible\:ring-destructive:focus-visible
.focus-visible\:ring-offset-2:focus-visible
.disabled\:opacity-50:disabled

/* Semantic utilities */
.bg-overlay  /* Modal backdrop: 50% background, adapta light/dark */
```

### shadcn/ui Components

Componentes baseados em **Radix UI** + **Tailwind CSS**, localizados em `src/components/ui/`:

| Componente | Arquivo | Descrição |
|------------|---------|-----------|
| **Button** | `button.tsx` | Variants: default, destructive, outline, secondary, ghost, link. Sizes: default, sm, lg, icon |
| **Card** | `card.tsx` | Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter |
| **Dialog** | `dialog.tsx` | Dialog, DialogTrigger, DialogContent, DialogHeader, DialogTitle, DialogDescription, DialogFooter |
| **Toast** | `toast.tsx` | Toast, ToastProvider, ToastViewport, ToastTitle, ToastDescription, ToastClose, ToastAction |
| **Switch** | `switch.tsx` | Toggle acessível (Radix Switch) |
| **Label** | `label.tsx` | Label acessível (Radix Label) |
| **Separator** | `separator.tsx` | Divisor visual (Radix Separator) |
| **Icons** | `Icons.tsx` | Lucide icons wrapper (loader, check, x, alertTriangle, etc.) |

### Utility: `cn()`

```typescript
// src/lib/utils.ts
import { clsx } from 'clsx'
import { twMerge } from 'tailwind-merge'

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}
```

---

## 🔔 Toast Notifications v2 (NOVO)

Sistema robusto, acessível e configurável implementado em `ToastContext.tsx` + `ToastContainer.tsx`.

### API

```typescript
// ToastContext.tsx
export type ToastType = 'success' | 'error' | 'info' | 'warning'

export interface ToastAction {
  label: string
  onClick: () => void
  variant?: 'primary' | 'secondary' | 'ghost'
}

export interface ToastOptions {
  type?: ToastType
  duration?: number          // default: 4000ms, 0 = persist
  action?: ToastAction
  richContent?: ReactNode
  persist?: boolean          // alias para duration: 0
  position?: ToastPosition   // default: 'top-right'
  onClose?: () => void
}

export interface Toast {
  id: number
  message: string
  type: ToastType
  options: ToastOptions
  createdAt: number
}

export interface ToastContextValue {
  toast: (message: string, options?: ToastOptions) => number
  success: (message: string, options?: Omit<ToastOptions, 'type'>) => number
  error: (message: string, options?: Omit<ToastOptions, 'type'>) => number
  info: (message: string, options?: Omit<ToastOptions, 'type'>) => number
  warning: (message: string, options?: Omit<ToastOptions, 'type'>) => number
  dismiss: (id: number) => void
  dismissAll: () => void
}
```

### Uso

```tsx
import { useToast } from '@/components/useToast'

function MyComponent() {
  const { success, error, toast } = useToast()

  const handleClick = () => {
    success('Backup criado com sucesso!')
    // ou com action
    error('Falha ao criar backup', {
      action: { label: 'Tentar novamente', onClick: () => handleRetry() }
    })
    // ou rich content
    toast('Processando...', {
      richContent: <ProgressBar value={50} />,
      persist: true
    })
  }
}
```

### Features Implementadas

| Feature | Descrição |
|---------|-----------|
| **Stack** | Até 5 toasts simultâneos (FIFO), gap 8px |
| **Animações** | Entrada: slide-in 300ms ease-out; Saída: slide-out 200ms ease-in |
| **Progress Bar** | Topo do toast, anima width 100%→0%, pausa no hover/focus |
| **Pause on Hover/Focus** | Countdown pausa, retoma ao sair |
| **Close Manual** | Botão X + tecla `Escape` |
| **Action Buttons** | `{ label, onClick, variant }` — executa callback + fecha |
| **Rich Content** | Renderiza `ReactNode` customizado no lugar da message |
| **Persist** | `persist: true` → sem auto-dismiss, sem progress bar |
| **6 Posições** | `top-right` (default), `top-left`, `top-center`, `bottom-right`, `bottom-left`, `bottom-center` |
| **Acessibilidade** | `role="status"` (info/success), `role="alert"` (error), `aria-live="polite"/"assertive"`, `aria-atomic`, focus management, teclado |

### UI/UX

```
┌─────────────────────────────────────────────┐
│ ██████████████████████████████████████████  │ ← Progress bar (top)
├─────────────────────────────────────────────┤
│ [Icon]  Mensagem do toast aqui...       [X] │
│              [Action Button]                │ ← opcional
└─────────────────────────────────────────────┘
```

- Border-left colorido por tipo (success=green, error=red, info=blue, warning=amber)
- Background: `bg-card` com `border-border`
- Shadow: `shadow-lg`

### Container Positions

```
top-left          top-center          top-right
  ┌─────┐           ┌─────┐           ┌─────┐
  │ ▼ 1 │           │ ▼ 1 │           │ ▼ 1 │
  │ ▼ 2 │           │ ▼ 2 │           │ ▼ 2 │
  └─────┘           └─────┘           └─────┘

bottom-left       bottom-center       bottom-right
  ┌─────┐           ┌─────┐           ┌─────┐
  │ ▲ 1 │           │ ▲ 1 │           │ ▲ 1 │
  │ ▲ 2 │           │ ▲ 2 │           │ ▲ 2 │
  └─────┘           └─────┘           └─────┘
```

---

## 🧭 Roteamento (React Router v7)

### Configuração (`src/router.tsx`)

```typescript
export const router = createBrowserRouter([
  {
    path: '/',
    element: <AppLayout />,
    children: [
      { index: true, element: <WorldListPage /> },
      { path: 'world/:folderName/backup/create', element: <CreateBackupPage /> },
      { path: 'world/:folderName/backups', element: <ListBackupsPage /> },
      { path: 'settings', element: <SettingsPage /> },  // NOVA
    ],
  },
]);
```

### Rotas

| Rota | Componente | Loader | Descrição |
|------|------------|--------|-----------|
| `/` | `WorldListPage` | `loadWorlds()` | Lista de mundos |
| `/world/:folderName/backup/create` | `CreateBackupPage` | `loadWorld(folderName)` | Criar backup |
| `/world/:folderName/backups` | `ListBackupsPage` | `loadBackups()` | Lista backups |
| `/settings` | `SettingsPage` | `loadConfig()` | **NOVA** Configurações |

### Layout (`AppLayout`)

```tsx
<div className="min-h-screen bg-background flex">
  {/* Sidebar - Desktop */}
  <aside className="w-64 bg-card border-r border-border flex flex-col hidden lg:flex">
    <nav className="flex-1 p-4 space-y-1">
      <NavLink to="/" className={({ isActive }) => ...}>
        <WorldIcon /> Mundos
      </NavLink>
      <NavLink to="/settings" className={({ isActive }) => ...}>
        <SettingsIcon /> Configurações
      </NavLink>
    </nav>
  </aside>

  {/* Mobile Header */}
  <header className="lg:hidden bg-card border-b border-border sticky top-0">
    <Link to="/">Blockoria</Link>
  </header>

  {/* Main Content */}
  <main className="flex-1 min-w-0">
    <div className="max-w-6xl mx-auto p-4 lg:p-6">
      <Outlet />
    </div>
  </main>
</div>
```

---

## 📱 Telas (MVP)

### 1. WorldList (`/`)

**Arquivo**: `src/components/WorldList.tsx`

**Props**:
```typescript
interface WorldListProps {
  worlds?: WorldSummaryDto[]
  onWorldSelect?: (world: WorldSummaryDto) => void
  onViewBackups?: (world: WorldSummaryDto) => void
}
```

**Features**:
- Grid responsivo de cards (`grid grid-cols-1 gap-4`)
- Card com: ícone (ou placeholder), nome, pasta, versão, badge (Compartilhado/Conta)
- Botão "Ver backups" → navega para `/world/:folderName/backups`
- Click no card inteiro → navega para CreateBackup
- Estados: Loading (spinner), Error (retry + toast), Empty (instruções + refresh)
- Acessibilidade: `role="list"`, `role="listitem"`, `tabIndex`, `Enter/Space`, focus visible

**Dados (Loader)**:
```typescript
async function loadWorlds() {
  const { invoke } = await import('@tauri-apps/api/core')
  return invoke<WorldSummaryDto[]>('cmd_list_worlds')
}
```

### 2. CreateBackup (`/world/:folderName/backup/create`)

**Arquivo**: `src/components/CreateBackup.tsx`

**Props**:
```typescript
interface CreateBackupProps {
  folderName: string
  levelName?: string
  onClose: () => void
  onSuccess?: (world: WorldSummaryDto) => void
}
```

**Fluxo**:
1. **Loading World** — `cmd_list_worlds` + filtra por `folderName`
2. **Idle** — Exibe info do mundo, tipo, versão, **diretório de destino real** (via `cmd_get_backup_root`)
3. **Loading** — Spinner no botão, desabilita inputs
4. **Success** — Toast verde, botões "Ver backups" / "Criar outro"
5. **Error** — Toast vermelho, botão "Tentar novamente"

**Diretório de Destino**:
- Busca o `backup_root` configurado via `cmd_get_backup_root` no mount
- Compõe: `{backup_root}\{account_id|Shared}\{folderName}\{timestamp}\`
- Fallback para `%APPDATA%\Blockoria\backups` se comando falhar

**Erros tratados**:
| Erro Backend | Mensagem UI |
|--------------|-------------|
| `InvalidWorldPath` / `World not found` | "Mundo não encontrado" |
| `Permission denied` | "Sem permissão para escrever no diretório de backup" |
| `Disk full` / `space` / `quota` | "Espaço insuficiente em disco" |
| Outros | "Erro ao criar backup: {msg}" |

### 3. ListBackups (`/world/:folderName/backups`)

**Arquivo**: `src/components/ListBackups.tsx`

**Props**:
```typescript
interface ListBackupsProps {
  worldFolderName: string
  levelName?: string
  accountId: string | null
  onClose: () => void
}
```

**Features**:
- Lista ordenada por data (mais recente primeiro)
- Card: data formatada (pt-BR), versão, caminho, botões Restaurar/Deletar
- **Modal Restaurar**: Confirmação com aviso "substituirá TODOS os arquivos", botão vermelho "Restaurar"
- **Modal Deletar**: Confirmação com aviso "NÃO PODE ser desfeita", botão vermelho "Deletar"
- **Toast** para sucesso/erro (substitui `alert()`)
- Empty state: "Nenhum backup encontrado para este mundo"
- Estados loading/error com retry
- Botão "Abrir pasta no Explorer" via `@tauri-apps/plugin-opener`

### 4. Settings (`/settings`) — **NOVA**

**Arquivo**: `src/pages/SettingsPage.tsx` + `src/components/Theme.tsx`

**Features**:
- **Theme Selector**: Radio group com 3 opções — Light, Dark, System
- **Persistência**: Salva em `%APPDATA%\Blockoria\config.toml` via `cmd_save_config`
- **Aplicação Imediata**: Atualiza CSS variables via `ThemeContext` → troca light/dark instantâneo
- **Sistema**: Respeita `prefers-color-scheme` quando "System" selecionado

```typescript
// ThemeContext.tsx
export type Theme = 'light' | 'dark' | 'system'

export interface ThemeContextValue {
  theme: Theme
  resolvedTheme: 'light' | 'dark'  // Tema efetivo aplicado
  setTheme: (theme: Theme) => void
}
```

---

## 🔌 Integração Tauri (IPC)

### Commands (`src-tauri/src/lib.rs`)

```rust
#[tauri::command]
async fn cmd_list_worlds(state: State<'_, AppState>) -> CommandResult<Vec<WorldSummaryDto>>

#[tauri::command]
async fn cmd_create_backup(
    world_folder_name: String,
    account_id: Option<String>,
    state: State<'_, AppState>
) -> CommandResult<CreateBackupResponseDto>

#[tauri::command]
async fn cmd_list_backups(
    world_folder_name: String,
    account_id: Option<String>,
    state: State<'_, AppState>
) -> CommandResult<Vec<BackupSummaryDto>>

#[tauri::command]
async fn cmd_restore_backup(
    backup_path: String,
    world_folder_name: String,
    account_id: Option<String>,
    state: State<'_, AppState>
) -> CommandResult<RestoreBackupResponseDto>

#[tauri::command]
async fn cmd_delete_backup(
    backup_path: String,
    state: State<'_, AppState>
) -> CommandResult<DeleteBackupResponseDto>

#[tauri::command]
async fn cmd_get_backup_root(state: State<'_, AppState>) -> CommandResult<String>

#[tauri::command]                                    // NOVO
async fn cmd_get_config(state: State<'_, AppState>) -> CommandResult<ConfigDto>

#[tauri::command]                                    // NOVO
async fn cmd_save_config(
    config: ConfigDto,
    state: State<'_, AppState>
) -> CommandResult<()>
```

### DTOs TypeScript (frontend)

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

// src/components/ToastContext.tsx (para config)
interface ConfigDto {
  theme: string  // 'light' | 'dark' | 'system'
}
```

### Invocação (frontend)

```typescript
import { invoke } from '@tauri-apps/api/core'

// Listar mundos
const worlds = await invoke<WorldSummaryDto[]>('cmd_list_worlds')

// Criar backup
const result = await invoke<CreateBackupResponseDto>('cmd_create_backup', {
  world_folder_name: world.folder_name,
  account_id: world.account_id,
})

// Listar backups
const backups = await invoke<BackupSummaryDto[]>('cmd_list_backups', {
  world_folder_name: 'world_1',
  account_id: null, // ou string
})

// Restaurar backup
await invoke<RestoreBackupResponseDto>('cmd_restore_backup', {
  backup_path: backup.backup_path,
  world_folder_name: backup.world_folder_name,
  account_id: accountId,
})

// Deletar backup
await invoke<DeleteBackupResponseDto>('cmd_delete_backup', {
  backup_path: backup.backup_path,
})

// Config - NOVO
const config = await invoke<ConfigDto>('cmd_get_config')
await invoke('cmd_save_config', { config: { theme: 'dark' } })
```

---

## 🧪 Testes

### Configuração Unit (Vitest)

**`vitest.config.ts`**:
```typescript
export default defineConfig({
  plugins: [react(), tailwindcss()],
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: ['./src/test/setup.ts'],
    include: ['src/**/*.{test,spec}.{ts,tsx}'],
  },
})
```

**`src/test/setup.ts`**:
```typescript
import { vi } from 'vitest'
import '@testing-library/jest-dom'

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))
vi.mock('@tauri-apps/plugin-dialog', () => ({ open: vi.fn(), save: vi.fn(), message: vi.fn() }))
vi.mock('@tauri-apps/plugin-fs', () => ({ readDir: vi.fn(), readFile: vi.fn(), writeFile: vi.fn() }))
vi.mock('@tauri-apps/plugin-opener', () => ({ openPath: vi.fn(), openUrl: vi.fn() }))
```

### Padrão de Testes (Given/When/Then)

```tsx
test('deve exibir loading e depois lista de mundos', async () => {
  // Given
  let resolveInvoke: (value: any) => void
  const invokePromise = new Promise((resolve) => { resolveInvoke = resolve })
  invokeMock.mockReturnValue(invokePromise)

  render(<WorldList />)

  // Then - loading
  await waitFor(() => {
    expect(screen.getByRole('status')).toHaveTextContent(/carregando/i)
  })

  // Resolve
  resolveInvoke!(mockWorlds)

  // Then - lista
  await waitFor(() => {
    expect(screen.getByText('Mundo Sobrevivência')).toBeInTheDocument()
  })
  expect(invokeMock).toHaveBeenCalledWith('cmd_list_worlds')
})
```

### Cobertura Unit

| Componente | Testes | Cenários |
|------------|--------|----------|
| WorldList | 6 | Loading, lista, empty, error, retry, keyboard nav |
| CreateBackup | 10 | Load world, create, success, errors (not found, permission, disk full), retry, cancel, Escape, keyboard |
| ListBackups | 8 | Load, list, empty, error, restore modal, delete modal, ordering, close |
| ToastContainer | 6 | Stack, animations, progress, hover pause, action, persist |
| ToastContext | 5 | Create, dismiss, dismissAll, types, options |
| useToast | 3 | Hook usage, provider check |
| useTheme | 2 | Theme toggle, persistence |
| **Total Unit** | **40** | |

```bash
cd app && bun test              # 40 passed
cd app && bun run build         # TypeScript + Vite build OK
```

### Testes E2E (Playwright) — **NOVO**

**`playwright.config.ts`**:
```typescript
export default defineConfig({
  testDir: './e2e',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: 'html',
  use: {
    baseURL: 'http://localhost:1420',
    trace: 'on-first-retry',
  },
  webServer: {
    command: 'cargo tauri dev',
    url: 'http://localhost:1420',
    reuseExistingServer: !process.env.CI,
    timeout: 120000,
  },
})
```

| Teste | Cenário |
|-------|---------|
| `list-worlds.spec.ts` | Carrega lista, navega para criar backup, navega para ver backups |
| `create-backup.spec.ts` | Cria backup, valida sucesso, valida erro mundo não encontrado |
| `list-backups.spec.ts` | Lista backups, ordenação, empty state |
| `restore-backup.spec.ts` | Restaura backup, confirmação modal, toast sucesso |
| `delete-backup.spec.ts` | Deleta backup, confirmação modal, idempotência |
| `full-flow.spec.ts` | Fluxo completo: listar → criar → listar → restaurar → deletar |

```bash
cd app && bun run test:e2e      # 6 passed (requer cargo tauri build antes)
```

---

## ⚙️ Configuração de Build

### Vite (`vite.config.ts`)

```typescript
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'
import { defineConfig } from 'vite'

export default defineConfig({
  plugins: [react(), tailwindcss()],
  server: {
    port: 1420,
    strictPort: true,
  },
})
```

### Tauri (`src-tauri/tauri.conf.json`)

```json
{
  "build": {
    "frontendDist": "../app/dist",
    "devUrl": "http://localhost:1420",
    "beforeDevCommand": "bun run --cwd ../app dev",
    "beforeBuildCommand": "bun run --cwd ../app build"
  }
}
```

### TypeScript (`tsconfig.app.json`)

```json
{
  "compilerOptions": {
    "verbatimModuleSyntax": true,
    "moduleResolution": "bundler",
    "module": "esnext",
    "types": ["vite/client", "@tauri-apps/api", "@testing-library/jest-dom"],
    "noUnusedLocals": true,
    "noUnusedParameters": true
  }
}
```

### shadcn/ui (`components.json`)

```json
{
  "$schema": "https://ui.shadcn.com/schema.json",
  "style": "new-york",
  "rsc": false,
  "tsx": true,
  "tailwind": {
    "config": "",
    "css": "src/index.css",
    "baseColor": "slate",
    "cssVariables": true,
    "prefix": ""
  },
  "aliases": {
    "components": "@/components",
    "utils": "@/lib/utils",
    "ui": "@/components/ui",
    "lib": "@/lib",
    "hooks": "@/hooks"
  }
}
```

---

## 🚀 Comandos de Desenvolvimento

```bash
# Frontend standalone (Vite dev server)
cd app && bun run dev
# → http://localhost:1420

# Integrado com Tauri (Rust + Frontend)
cargo tauri dev
# → Compila Rust, inicia Vite, abre janela Tauri

# Build produção
cd app && bun run build
# → dist/ gerado

# Build completo Tauri
cargo tauri build
# → target/release/bundle/... (MSI/EXE)

# Testes
cd app && bun test              # Vitest unit (40 passed)
cd app && bun run test:e2e      # Playwright E2E (6 passed)
cargo test                      # Backend (223 passed)
```

---

## 📦 Dependências Principais

```json
{
  "dependencies": {
    "react": "^19.2.8",
    "react-dom": "^19.2.8",
    "react-router-dom": "^7.18.3",
    "@tauri-apps/api": "^2.11.1",
    "@tauri-apps/plugin-dialog": "^2.7.3",
    "@tauri-apps/plugin-fs": "^2.5.2",
    "@tauri-apps/plugin-opener": "^2.5.5",
    "tailwindcss": "^4.3.3",
    "@tailwindcss/vite": "^4.3.3",
    "clsx": "^2.1.1",
    "tailwind-merge": "^2.5.4",
    "lucide-react": "^0.468.0",
    "sonner": "^1.7.0",
    "class-variance-authority": "^0.7.1"
  },
  "devDependencies": {
    "vite": "^8.3.0",
    "@vitejs/plugin-react": "^6.1.1",
    "vitest": "^5.0.0",
    "@testing-library/react": "^16.3.3",
    "@testing-library/user-event": "^14.6.7",
    "jsdom": "^30.0.1",
    "typescript": "~6.0.2",
    "@playwright/test": "^1.48.0"
  }
}
```

---

## 🔮 Próximos Passos (Pós-MVP)

| Item | Prioridade |
|------|------------|
| Toast "promise" (auto-show loading → success/error) | Média |
| Histórico de toasts (notification center) | Baixa |
| Sons/notificações sonoras | Baixa |
| Integração com system notifications (Web Notifications API) | Baixa |
| Drag & drop para restaurar | Baixa |
| Preview do backup antes de restaurar | Baixa |
| Configurações avançadas (diretório de backup, auto-backup) | Média |
| Internacionalização (i18n) | Baixa |

---

## 📚 Referências

- [SDD Specs](../specs/) — Especificações das 4 telas + NBT + Toast
- [BDD Features](../features/) — Cenários Gherkin
- [Domain Docs](domain.md)
- [Application Docs](application.md)
- [Tauri v2 Docs](https://tauri.app/v2/)
- [React Router v7 Docs](https://reactrouter.com/)
- [Tailwind CSS v4 Docs](https://tailwindcss.com/docs)
- [shadcn/ui Docs](https://ui.shadcn.com/)
- [Playwright Docs](https://playwright.dev/)
