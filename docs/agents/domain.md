# Domain Documentation Configuration

## Layout

Single-context

## Files

| File | Purpose |
|------|---------|
| `CONTEXT.md` | Root domain context — glossary, bounded context boundaries, key invariants, integration contracts |
| `docs/adr/` | Architecture Decision Records — one file per decision, named `NNNN-short-title.md` |

## Consumer Rules

- **Agents** (`domain-modeling`, `grill-with-docs`, `to-spec`, `to-tickets`): read `CONTEXT.md` first, then relevant ADRs
- **Humans**: same order; `CONTEXT.md` is the entry point
- **Writes**: `domain-modeling` updates `CONTEXT.md`; `grill-with-docs` creates ADRs in `docs/adr/`
- **Never** edit ADRs after creation — supersede with a new ADR instead

## Conventions

- `CONTEXT.md` sections: Glossary, Bounded Contexts, Key Invariants, Integration Contracts, Open Questions
- ADR format: Title, Status, Context, Decision, Consequences, Alternatives Considered
- ADR numbering: sequential 4-digit (0001, 0002, ...)

## ADR Template

See `docs/adr/0000-template.md` (create on first ADR if missing).
