# ADR-0001: Taxonomia de erros por camada (`ApplicationError`)

## Status

Accepted (2026-10-19)

## Context

Até a v0.9.1, `DomainError` era o tipo de erro universal de todo o backend: os ports, os use cases, a infraestrutura e até o adapter Tauri retornavam exclusivamente variants de **validação de domínio** para qualquer tipo de falha. As consequências observadas no código:

- `impl From<io::Error> for DomainError` convertia **qualquer** I/O em `InvalidBackupPath` — ler `config.toml`, iterar `read_dir` e travessia de lock apareciam como "erro de validação de path de backup".
- A camada application não tinha vocabulário próprio: "backup não encontrado" (`InvalidBackupPath("Backup not found")`), "mundo não encontrado" (`InvalidWorldPath("World not found")`) e falha de cópia de arquivos eram todos indistinguíveis de validação.
- Sem categorias, nenhum boundary consegue decidir nada: o `CommandError` do Tauri serializava a string de debug (`"Domain error: InvalidWorldPath: ..."`) direto para o frontend, que não tinha como distinguir programaticamente "não achei" de "sem permissão de disco".
- Categorias ausentes = mensagens genéricas ou vazamento de internals em inglês na UI (o WorldList inclusive descartava a mensagem real por um bug de `instanceof Error`).

Especificação completa: `docs/specs/07-error-handling.md`.

## Decision

1. **`blockoria-application` ganha `ApplicationError`** (thiserror) como tipo de erro dos ports e use cases, com exatamente 4 categorias:
   - `NotFound { what: What::World | What::Backup }` — agregado procurado não existe
   - `Validation(#[from] DomainError)` — regra de domínio violada
   - `Storage(#[from] io::Error)` — falha de I/O real (permissão, disco, path)
   - `Config(String)` — ler/gravar/serializar `config.toml`
2. **`DomainError` volta a ser puramente validação**: o `From<io::Error>` é removido. I/O só entra no sistema por um lugar: `ApplicationError::Storage`.
3. **O contrato IPC é estruturado**: `CommandError` serializa como objeto `{ "code", "message" }`, com `code ∈ not_found | validation | storage | config | unknown` mapeando 1:1 as categorias; `message` permanece técnica (inglês, para log).
4. **PT-BR mora só no frontend**: `tauriError.ts` extrai `{code, message}` de qualquer rejeição (nunca descarta o payload) e `humanMessage` mapeia `code` → mensagem humana. Sem framework de i18n — um `Record`.
5. **Falhas de startup não morrem caladas**: `main.rs` exibe message box nativo (`rfd`) com o erro (incluindo o caminho procurado) antes de encerrar com código ≠ 0.
6. **Crash de render não vira tela branca**: `ErrorBoundary` global + `errorElement` no router com fallback compartilhado (`ErrorScreen`).

## Consequences

- **Positivas:** o frontend consegue reagir por categoria (mensagem certa, retry onde faz sentido); logs deixam de mentir sobre a causa; "not found" é pesquisável em log/telemetria futura; a degradação de dados parciais (NBT ilegível → versão default etc.) ficou explícita na spec 07 em vez de implícita.
- **Negativas / custos:** migração mecânica em toda a cadeia (ports → use cases → infra → adapter) feita num único commit pois traits e impls têm que andar juntos; testes de contrato e mocks reescritos para o novo tipo; o diff tocou 13 arquivos no commit de migração.
- **Teto conhecido:** `What` tem só `World | Backup` — novas entidades exigirão estender o enum (mudança local, sem quebra de contrato IPC pois o `code` continua `not_found`).

## Alternatives Considered

| Alternativa | Por que rejeitada |
|-------------|-------------------|
| Manter `DomainError` universal, adicionando variants | Cresceria o enum com casos não-validação; a categoria continuaria ambígua no boundary |
| `anyhow` no core | Perde a categorização programática que o frontend precisa; erros viram opacos |
| Tipo de erro por use case | Explosão de tipos para 4 operações de app local; YAGNI |
| Remapear no adapter (Tauri) com categorias, mantendo ports com `DomainError` | As categorias nasceriam sem informação: o I/O já teria sido esmagado em `InvalidBackupPath` antes de chegar ao adapter |

## Referências

- Spec: [`docs/specs/07-error-handling.md`](../specs/07-error-handling.md)
- BDD: [`tests/features/07-error-handling.feature`](../../tests/features/07-error-handling.feature)
- Docs: [`docs/application.md`](../application.md) · [`docs/frontend.md`](../frontend.md) · [`docs/domain.md`](../domain.md)
