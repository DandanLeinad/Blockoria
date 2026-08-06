// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

use crate::DomainError;
use std::path::{Path, PathBuf};

/// Value Object para caminho de pasta de backup.
///
/// Representa o diretório onde um backup é armazenado.
/// Valida que o caminho existe no filesystem e é um diretório válido.
///
/// Regra de validação:
/// - Não vazio
/// - Não "." (diretório atual)
/// - Deve existir no filesystem
/// - Deve ser um diretório
///
/// # Exemplos
///
/// ```
/// use blockoria_domain::BackupPath;
/// use tempfile::tempdir;
/// use std::path::PathBuf;
///
/// let dir = tempdir().unwrap();
/// let path = BackupPath::new(dir.path()).unwrap();
/// assert!(path.as_path().exists());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupPath(PathBuf);

impl BackupPath {
    /// Cria um novo BackupPath validando o filesystem.
    ///
    /// # Erros
    ///
    /// Retorna `DomainError::InvalidBackupPath` se:
    /// - Caminho vazio
    /// - Caminho é "." (diretório atual)
    /// - Caminho não existe no filesystem
    /// - Caminho não é um diretório
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, DomainError> {
        let path = path.into();

        // Rejeita "." explicitamente
        if path == Path::new(".") {
            return Err(DomainError::InvalidBackupPath(
                "Backup path cannot be current directory".into(),
            ));
        }

        if !path.exists() {
            return Err(DomainError::InvalidBackupPath(
                "Backup path does not exist".into(),
            ));
        }

        if !path.is_dir() {
            return Err(DomainError::InvalidBackupPath(
                "Backup path must be a directory".into(),
            ));
        }

        Ok(BackupPath(path))
    }

    /// Retorna o caminho como referência a `Path`.
    pub fn as_path(&self) -> &Path {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn given_existing_dir_when_new_then_ok() {
        // Given
        let temp_dir = tempdir().unwrap();
        let path = temp_dir.path();

        // When
        let result = BackupPath::new(path);

        // Then
        assert!(result.is_ok());
    }

    #[test]
    fn given_empty_when_new_then_err() {
        // Given
        let path = PathBuf::from("");

        // When
        let result = BackupPath::new(path);

        // Then
        assert!(matches!(result, Err(DomainError::InvalidBackupPath(_))));
    }

    #[test]
    fn given_dot_when_new_then_err() {
        // Given
        let path = PathBuf::from(".");

        // When
        let result = BackupPath::new(path);

        // Then
        assert!(matches!(result, Err(DomainError::InvalidBackupPath(_))));
    }

    #[test]
    fn given_not_exists_when_new_then_err() {
        // Given
        let path = PathBuf::from("/path/that/does/not/exist");

        // When
        let result = BackupPath::new(path);

        // Then
        assert!(matches!(result, Err(DomainError::InvalidBackupPath(_))));
    }

    #[test]
    fn given_file_not_dir_when_new_then_err() {
        // Given
        let temp_file = tempfile::NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // When
        let result = BackupPath::new(path);

        // Then
        assert!(matches!(result, Err(DomainError::InvalidBackupPath(_))));
    }
}
