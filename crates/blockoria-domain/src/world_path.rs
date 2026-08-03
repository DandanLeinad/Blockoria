// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

use crate::DomainError;
use std::path::{Path, PathBuf};

/// Value Object para caminho de mundo Minecraft Bedrock.
///
/// Valida que o caminho:
/// - Não é vazio
/// - Existe no filesystem
/// - É um diretório (não arquivo)
///
/// # Exemplos
///
/// ```
/// use blockoria_domain::WorldPath;
/// use tempfile::tempdir;
/// let dir = tempdir().unwrap();
/// let path = WorldPath::new(dir.path()).unwrap();
/// assert!(path.as_path().exists());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorldPath(PathBuf);

impl WorldPath {
    /// Cria um novo WorldPath validando o filesystem.
    ///
    /// # Erros
    ///
    /// Retorna `DomainError::InvalidWorldPath` se:
    /// - Caminho vazio
    /// - Caminho é "." (diretório atual)
    /// - Caminho não existe
    /// - Caminho não é um diretório
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, DomainError> {
        let path = path.into();
        // Rejeita "." explicitamente (como Python)
        if path == Path::new(".") {
            return Err(DomainError::InvalidWorldPath(
                "World path cannot be current directory".into(),
            ));
        }
        if !path.exists() {
            return Err(DomainError::InvalidWorldPath(
                "World path does not exist".into(),
            ));
        }
        if !path.is_dir() {
            return Err(DomainError::InvalidWorldPath(
                "World path must be a directory".into(),
            ));
        }
        Ok(WorldPath(path))
    }

    /// Retorna o caminho como referência a `Path`.
    pub fn as_path(&self) -> &Path {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use tempfile::tempdir; // para criar dirs temporários nos testes

    #[test]
    fn given_existing_dir_when_new_then_ok() {
        // Given
        let temp_dir = tempdir().unwrap();
        let path = temp_dir.path();

        // When
        let result = super::WorldPath::new(path);

        // Then
        assert!(result.is_ok());
    }

    #[test]
    fn given_empty_when_new_then_err() {
        // Given
        let path = PathBuf::from("");

        // When
        let result = super::WorldPath::new(path);

        // Then
        assert!(result.is_err());
    }

    #[test]
    fn given_not_exists_when_new_then_err() {
        // Given
        let path = PathBuf::from("/path/that/does/not/exist");

        // When
        let result = super::WorldPath::new(path);

        // Then
        assert!(result.is_err());
    }

    #[test]
    fn given_file_not_dir_when_new_then_err() {
        // Given
        let temp_file = tempfile::NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // When
        let result = super::WorldPath::new(path);

        // Then
        assert!(result.is_err());
    }

    #[test]
    fn given_dot_when_new_then_err() {
        // Given
        let path = PathBuf::from(".");

        // When
        let result = super::WorldPath::new(path);

        // Then
        assert!(result.is_err());
    }
}
