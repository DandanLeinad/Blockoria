# Spec: Toast Notifications System (v2)

## Visão Geral
Sistema de toast notifications robusto, acessível e configurável para substituir a implementação básica atual.

---

## Requisitos Funcionais

### RF-01: Toast Container com Stack
- **Dado** que múltiplos toasts são criados em sequência
- **Quando** renderizados
- **Então** devem empilhar verticalmente com gap de 8px
- **E** máximo 5 toasts visíveis simultaneamente (FIFO: remove o mais antigo)

### RF-02: Animações de Entrada e Saída
- **Entrada**: `slide-in` (translateX 100% → 0, opacity 0 → 1) — 300ms ease-out (já existe)
- **Saída**: `slide-out` (translateX 0 → 100%, opacity 1 → 0) — 200ms ease-in
- **Quando** auto-dismiss ou close manual → animação de saída antes de remover do DOM

### RF-03: Progress Bar Visual
- **Dado** toast com duração definida (default 4000ms)
- **Quando** toast está visível
- **Então** barra de progresso (topo do toast) anima width 100% → 0%
- **E** pausa quando hover/focus no toast

### RF-04: Pause on Hover/Focus
- **Dado** toast com auto-dismiss
- **Quando** usuário faz hover OU foca (Tab) no toast
- **Então** countdown pausa
- **Quando** remove hover/focus
- **Então** countdown retoma do ponto pausado

### RF-05: Close Manual
- Botão "X" no canto direito
- Click → inicia animação de saída → remove do DOM
- Tecla `Escape` com foco no toast → fecha

### RF-06: Action Buttons (Opcional)
- **Dado** toast com `action: { label: "Desfazer", onClick: fn }`
- **Quando** renderizado
- **Então** mostra botão secundário ao lado do close
- Click na action → executa callback → fecha toast

### RF-07: Rich Content Support
- **Dado** `richContent: <ReactNode>` no options
- **Quando** renderizado
- **Então** renderiza o node no lugar da message string
- **E** message string vira fallback para screen readers

### RF-08: Persist Option
- **Dado** `persist: true` no options
- **Quando** toast criado
- **Então** não auto-dismiss (precisa close manual ou action)
- **E** progress bar não aparece

### RF-09: Posições Configuráveis
| Posição | CSS |
|---------|-----|
| `top-right` | `top-4 right-4` (default) |
| `top-left` | `top-4 left-4` |
| `bottom-right` | `bottom-4 right-4` |
| `bottom-left` | `bottom-4 left-4` |
| `top-center` | `top-4 left-1/2 -translate-x-1/2` |
| `bottom-center` | `bottom-4 left-1/2 -translate-x-1/2` |

### RF-10: Acessibilidade Completa
- `role="status"` para info/success, `role="alert"` para error
- `aria-live="polite"` (info/success), `aria-live="assertive"` (error)
- `aria-atomic="true"` — screen reader lê toast completo
- Focus management: ao abrir, foca no close button (opcional, via prop)
- Teclado: `Escape` fecha, `Tab` navega entre close/action

---

## API Proposta

```typescript
// ToastContext.tsx — Types estendidos
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
  toast: (message: string, options?: ToastOptions) => number  // retorna id
  success: (message: string, options?: Omit<ToastOptions, 'type'>) => number
  error: (message: string, options?: Omit<ToastOptions, 'type'>) => number
  info: (message: string, options?: Omit<ToastOptions, 'type'>) => number
  warning: (message: string, options?: Omit<ToastOptions, 'type'>) => number
  dismiss: (id: number) => void
  dismissAll: () => void
}
```

---

## UI/UX

### Toast Individual
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

## Critérios de Aceite

1. **Stack**: 5 toasts rápidos → empilham, 6º remove o 1º
2. **Animação saída**: fade+slide 200ms antes de unmount
3. **Progress bar**: anima linear, pausa no hover, retoma no leave
4. **Hover pause**: countdown para, retoma ao sair
5. **Action button**: executa callback + fecha toast
6. **Rich content**: renderiza JSX customizado
7. **Persist**: sem progress bar, só fecha manual/action
8. **Posições**: 6 posições funcionais via prop no Provider
9. **A11y**: screen reader anuncia corretamente, teclado navega
10. **Performance**: zero re-renders desnecessários (memo, useCallback)

---

## Fora de Escopo
- Toast "promise" (auto-show loading → success/error)
- Histórico de toasts (notification center)
- Sons/notificações sonoras
- Integração com system notifications (Web Notifications API)
