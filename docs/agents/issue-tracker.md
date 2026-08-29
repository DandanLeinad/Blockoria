# Issue Tracker Configuration

## Tracker Type

GitHub Issues

## Repository

DandanLeinad/blockoria

## CLI

`gh` (GitHub CLI)

## Workflow

- Issues are created, updated, and closed via `gh issue` commands
- Labels are managed via `gh label` commands
- PRs are **not** automatically included in the triage queue (set `prs_as_request_surface: true` in this file to enable)

## Conventions

- Issue titles: imperative mood, lowercase first letter ("add feature X", not "Add feature X")
- Labels: use the five canonical triage labels (see `triage-labels.md`) plus domain labels as needed
- Milestones: optional, used for release planning
