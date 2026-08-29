// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! Backup repository port — read/write operations for backups.
//!
//! Defines the `BackupRepository` trait which provides read and write access
// to backup aggregates. The infrastructure layer must implement this trait to
// provide backup persistence from any storage backend.

use blockoria_domain::{AccountId, Backup, BackupPath, DomainError, WorldFolderName};

/// Port for reading and writing Backup aggregates.
pub trait BackupRepository: Send + Sync {
    /// Persists a backup.
    ///
    /// Currently unused; reserved for future use cases that need to
    /// create backup records without filesystem operations (e.g. importing
    /// existing backups, cloud sync metadata).
    fn save(&self, backup: &Backup) -> Result<(), DomainError>;

    /// Returns all backups for a given world (folder_name + account_id).
    fn list_by_world(
        &self,
        folder_name: &WorldFolderName,
        account_id: &AccountId,
    ) -> Result<Vec<Backup>, DomainError>;

    /// Deletes a backup by its backup path.
    fn delete(&self, backup_path: &BackupPath) -> Result<(), DomainError>;
}
