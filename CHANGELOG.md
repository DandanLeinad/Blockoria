# Changelog

## [0.2.0] - 2026-08-16

### Features
- feat: complete blockoria-domain with 8 VOs, 2 aggregates, 64 tests
- feat: add WorldFolderName, LevelName, WorldPath, AccountId, WorldVersion, WorldIconPath, BackupTimestamp, BackupPath VOs
- feat: add World and Backup aggregates
- feat: add DomainError enum with 8 variants
- docs: add README.md with project overview and legal disclaimer

### Bug Fixes
- fix: pre-commit hooks with uv venv
- fix: DCO check skips merge commits

### CI/CD
- ci: add unified CI workflow (fmt, check, test, clippy, doc, pre-commit)
- ci: set publish = false on all workspace crates
- ci: remove duplicate tests.yml workflow

### Documentation
- docs: simplify to Portuguese-only, clean zensical config
- docs: fix VO count to 8

## [0.1.0] - 2026-07-30

### Features
- chore: initial workspace setup with clean architecture
- feat(domain): add DomainError enum with Display and Error trait
- feat(domain): add WorldFolderName VO with validation
- feat(domain): add WorldPath VO with filesystem validation
- feat(domain): add LevelName VO with validation
- feat(domain): add WorldModel VOs (AccountId, WorldVersion, IconPath)
- feat(domain): add BackupTimestamp and BackupPath VOs
- feat(domain): complete Domain Layer for CreateBackup
- ci: add Tests & Quality and Security Audit workflows
