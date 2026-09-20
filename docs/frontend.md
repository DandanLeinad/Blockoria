---
icon: lucide/monitor
---

# Frontend (Tauri + React + TypeScript)

Documentação técnica da interface gráfica do Blockoria.

> **Status**: ✅ Implementado — 3 telas MVP, 24 testes, integração Tauri completa.

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
├── tsconfig.json           # Project references
├── tsconfig.app.json       # App config (verbatimModuleSyntax)
├── src/
│   ├── main.tsx                    # Entry point → RouterProvider
│   ├── App.tsx                     # <RouterProvider router={router} />
│   ├── router.tsx                  # React Router v7 config
│   ├── index.css                   # Tailwind v4 theme (CSS variables)
│   ├── components/
│   │   ├── WorldList.tsx           # Tela 1: Lista de mundos
│   │   ├── CreateBackup.tsx        # Tela 2: Criar backup
│   │   └── ListBackups.tsx         # Tela 3: Listar/restaurar/deletar
│   └── test/
│       └── setup.ts                # Vitest setup (mocks @tauri-apps/api)
├── public/
└── index.html
```

---

## 🎨 Design System (Tailwind v4)

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
/* Backgrounds */
.bg-background, .bg-foreground, .bg-card, .bg-card-foreground
.bg-popover, .bg-popover-foreground
.bg-primary, .bg-primary-foreground
.bg-secondary, .bg-secondary-foreground
.bg-muted, .bg-muted-foreground
.bg-accent, .bg-accent-foreground
.bg-destructive, .bg-destructive-foreground
.bg-success, .bg-success-foreground
.bg-warning, .bg-warning-foreground

/* Text */
.text-foreground, .text-muted-foreground, .text-primary
.text-primary-foreground, .text-secondary-foreground
.text-destructive, .text-destructive-foreground
.text-success, .text-warning

/* Borders */
.border-border, .border-input, .border-ring

/* Radius */
.rounded, .rounded-lg, .rounded-xl, .rounded-full

/* Shadows */
.shadow-sm, .shadow, .shadow-md, .shadow-lg, .shadow-xl

/* Transitions */
.transition-all, .transition-colors, .transition-shadow

/* Hover/Focus states */
.hover\:bg-primary\/90:hover
.hover\:bg-secondary:hover
.hover\:text-primary:hover
.focus-visible\:ring-2:focus-visible
.disabled\:opacity-50:disabled
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

### Layout (`AppLayout`)

```tsx
<div className="min-h-screen bg-background flex">
  {/* Sidebar - Desktop */}
  <aside className="w-64 bg-card border-r border-border flex flex-col hidden lg:flex">
    <nav className="flex-1 p-4 space-y-1">
      <NavLink to="/" className={({ isActive }) => ...}>
        <WorldIcon /> Mundos
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
  worlds?: WorldSummaryDto[]      // Do loader (opcional)
  onWorldSelect?: (world) => void
  onViewBackups?: (world) => void
}
```

**Features**:
- Grid responsivo de cards (`grid grid-cols-1 gap-4`)
- Card com: ícone (ou placeholder), nome, pasta, versão, badge (Compartilhado/Conta)
- Botão "Ver backups" → navega para `/world/:folderName/backups`
- Click no card inteiro → navega para CreateBackup
- Estados: Loading (spinner), Error (retry), Empty (instruções + refresh)
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
  onClose: () => void
  onSuccess?: (world: WorldSummaryDto) => void
}
```

**Fluxo**:
1. **Loading World** — `cmd_list_worlds` + filtra por `folderName`
2. **Idle** — Exibe info do mundo, tipo, versão, diretório de destino
3. **Loading** — Spinner no botão, desabilita inputs
4. **Success** — Toast verde, botões "Ver backups" / "Criar outro"
5. **Error** — Toast vermelho, botão "Tentar novamente"

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
  accountId: string | null
  onClose: () => void
  onRestore?: (backup) => void
  onDelete?: (backup) => void
}
```

**Features**:
- Lista ordenada por data (mais recente primeiro)
- Card: data formatada (pt-BR), versão, caminho, botões Restaurar/Deletar
- **Modal Restaurar**: Confirmação com aviso "substituirá TODOS os arquivos", botão vermelho "Restaurar"
- **Modal Deletar**: Confirmação com aviso "NÃO PODE ser desfeita", botão vermelho "Deletar"
- Empty state: "Nenhum backup encontrado para este mundo"
- Estados loading/error com retry

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
```

---

## 🧪 Testes

### Configuração

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

### Cobertura

| Componente | Testes | Cenários |
|------------|--------|----------|
| WorldList | 6 | Loading, lista, empty, error, retry, keyboard nav |
| CreateBackup | 10 | Load world, create, success, errors (not found, permission, disk full), retry, cancel, Escape, keyboard |
| ListBackups | 8 | Load, list, empty, error, restore modal, delete modal, ordering, close |

```bash
cd app && bun test
# 24 passed
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
cd app && bun test              # Vitest
cargo test                      # Backend
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
    "@tailwindcss/vite": "^4.3.3"
  },
  "devDependencies": {
    "vite": "^8.3.0",
    "@vitejs/plugin-react": "^6.1.1",
    "vitest": "^5.0.0",
    "@testing-library/react": "^16.3.3",
    "@testing-library/user-event": "^14.6.7",
    "jsdom": "^30.0.1",
    "typescript": "~6.0.2"
  }
}
```

---

## 🔮 Próximos Passos (Pós-MVP)

| Item | Prioridade |
|------|------------|
| Toast notifications (substituir `alert()`) | Alta |
| shadcn/ui components (Dialog, Toast, Button, Card) | Média |
| Drag & drop para restaurar | Baixa |
| Preview do backup antes de restaurar | Baixa |
| Configurações (diretório de backup, auto-backup) | Média |
| Internacionalização (i18n) | Baixa |

---

## 📚 Referências

- [SDD Specs](../specs/) — Especificações das 4 telas + NBT
- [BDD Features](../features/) — Cenários Gherkin
- [Domain Docs](domain.md)
- [Application Docs](application.md)
- [Tauri v2 Docs](https://tauri.app/v2/)
- [React Router v7 Docs](https://reactrouter.com/)
- [Tailwind CSS v4 Docs](https://tailwindcss.com/docs)
