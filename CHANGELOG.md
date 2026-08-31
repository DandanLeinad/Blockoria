# Changelog

## [0.5.0] - 2026-08-31

### Features
- feat(infrastructure): add FileWorldRepository with explicit directory traversal (20 tests)
- feat(infrastructure): add FileBackupRepository with timestamp-based structure (20 tests)
- feat(infrastructure): add Config system with config.toml at %APPDATA%\Blockoria\config.toml
- feat(domain): add WorldLocation enum (Account|Shared) with 11 TDD tests
- feat(domain): add BackupTimestamp::from_filename_safe for filename-safe parsing
- feat(domain): add Ord impl for BackupTimestamp (sort backups by timestamp)
- feat(domain): add Default for WorldVersion ([0,0,0,0,0])
- feat(domain): add Hash impl for AccountId (HashMap key support)
- feat(domain): add From<io::Error> for DomainError (ergonomic error conversion)
- feat(application): update all 5 use cases and ports for WorldLocation

### Documentation
- docs: update domain.md with WorldLocation and BackupTimestamp changes
- docs: update application.md with WorldLocation in use cases signatures
- docs: update Infrastructure layer status to Implemented (41 tests)
- docs: update README.md and docs/index.md with Infrastructure status
- docs: correct Domain test count to 78 (66 unit + 12 doctests)
- docs: update workspace structure in README and docs

### CI/CD
- ci: fix cargo-deny license validation (AGPL-3.0 -> AGPL-3.0-only)
- ci: add MPL-2.0 to allowed licenses for option-ext dependency

### Dependencies
- deps: add toml for config parsing

## [0.4.0] - 2026-08-29

### Features
- feat(application): add complete Application layer with 5 use cases (14 tests)
- feat(application): add WorldRepository and BackupRepository ports
- feat(application): add util::copy_dir_all shared utility (used by create_backup and restore_backup)

### Documentation
- docs: add domain.md and application.md reference pages
- docs: update Application layer status to Implemented (5 use cases, 14 tests)
- docs: correct Domain test count to 65 (54 unit + 11 doctests)
- docs: add agent skills configuration (issue-tracker.md, domain.md)
- docs: update Application layer status to Implemented (5 use cases)
- docs: update workspace structure in README
- docs: correct Domain test count to 65 (54 unit + 11 doctests)

### CI/CD
- ci: upgrade cargo-deny action to v2 with rust-version: stable
- ci: optimize security audit with cargo-binstall (pre-built binaries)
- ci: add cargo report future-incompatibilities step
- ci: add concurrency with cancel-in-progress to security audit
- ci: fix security audit workflow job ID (security-audit) and name (Security Audit)
- ci: fix security workflow jq parsing for all JSON formats
- ci: add rust-toolchain.toml (stable channel with rustfmt, clippy)

### Dependencies
- deps: bump zensical to 0.0.57

## [0.3.0] - 2026-08-26

### Features
- feat(application): add blockoria-application crate with create_backup use case
- feat(application): add iterative copy_dir_all to prevent stack overflow
- feat(application): add 5 integration tests for backup creation

### Bug Fixes
- fix: cargo-deny license identifier (AGPL-3.0-or-later -> AGPL-3.0)
- fix: security audit workflow jq parsing for all JSON formats
- fix: cargo-binstall cache conflict by removing ~/.cargo/bin from cache

### CI/CD
- ci: upgrade cargo-deny action to v2 with rust-version: stable
- ci: optimize security audit with cargo-binstall (pre-built binaries)
- ci: add cargo report future-incompatibilities step
- ci: add concurrency with cancel-in-progress to security audit
- ci: add rust-toolchain.toml (stable channel with rustfmt, clippy)

### Documentation
- docs: update Application layer status to started (create_backup implemented)
- docs: correct Domain test count to 65 (54 unit + 11 doctests)
- docs: update next steps to reflect remaining Application work

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
