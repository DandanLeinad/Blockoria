// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

use crate::DomainError;
use std::path::{Path, PathBuf};

/// Value Object para caminho do ícone do mundo Minecraft.
///
/// Representa o arquivo `world_icon.jpeg` dentro da pasta do mundo.
/// Pode ser `None` se o mundo não tiver ícone (feature desativada).
///
/// Regra de validação (quando `Some`):
/// - Arquivo deve se chamar exatamente `world_icon.jpeg`
/// - Caminho não pode ser vazio
///
/// # Exemplos
///
/// ```
/// use blockoria_domain::WorldIconPath;
/// use std::path::PathBuf;
///
/// // Com ícone válido
/// let icon = WorldIconPath::new(Some(PathBuf::from("world_icon.jpeg"))).unwrap();
/// assert!(icon.is_some());
///
/// // Sem ícone
/// let no_icon = WorldIconPath::new(None::<PathBuf>).unwrap();
/// assert!(no_icon.is_none());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorldIconPath(Option<PathBuf>);

impl WorldIconPath {
    /// Cria um novo WorldIconPath validando o formato.
    ///
    /// # Erros
    ///
    /// Retorna `DomainError::InvalidWorldIconPath` se:
    /// - `Some(path)` onde filename não é `world_icon.jpeg`
    /// - `Some(path)` onde path é vazio
    pub fn new(value: Option<impl Into<PathBuf>>) -> Result<Self, DomainError> {
        match value {
            Some(path) => {
                let path = path.into();
                if path.as_os_str().is_empty() {
                    return Err(DomainError::InvalidWorldIconPath(
                        "World icon path cannot be empty".into(),
                    ));
                }
                let file_name = path.file_name().and_then(|s| s.to_str()).ok_or_else(|| {
                    DomainError::InvalidWorldIconPath("World icon path has invalid filename".into())
                })?;
                if file_name != "world_icon.jpeg" {
                    return Err(DomainError::InvalidWorldIconPath(
                        "World icon must be named 'world_icon.jpeg'".into(),
                    ));
                }
                Ok(WorldIconPath(Some(path)))
            }
            None => Ok(WorldIconPath(None)),
        }
    }

    /// Retorna `true` se tem ícone.
    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    /// Retorna `true` se não tem ícone.
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }

    /// Retorna o caminho interno como `Option<&Path>`.
    pub fn as_path(&self) -> Option<&Path> {
        self.0.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn given_none_when_new_then_ok() {
        // Given
        let input = None::<PathBuf>;

        // When
        let result = WorldIconPath::new(input);

        // Then
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn given_valid_filename_when_new_then_ok() {
        // Given
        let input = Some(PathBuf::from("world_icon.jpeg"));

        // When
        let result = WorldIconPath::new(input);

        // Then
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn given_valid_nested_path_when_new_then_ok() {
        // Given
        let input = Some(PathBuf::from("folder/world_icon.jpeg"));

        // When
        let result = WorldIconPath::new(input);

        // Then
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn given_wrong_filename_when_new_then_err() {
        // Given
        let input = Some(PathBuf::from("icon.png"));

        // When
        let result = WorldIconPath::new(input);

        // Then
        assert!(matches!(result, Err(DomainError::InvalidWorldIconPath(_))));
    }

    #[test]
    fn given_wrong_extension_when_new_then_err() {
        // Given
        let input = Some(PathBuf::from("world_icon.jpg"));

        // When
        let result = WorldIconPath::new(input);

        // Then
        assert!(matches!(result, Err(DomainError::InvalidWorldIconPath(_))));
    }

    #[test]
    fn given_empty_path_when_new_then_err() {
        // Given
        let input = Some(PathBuf::from(""));

        // When
        let result = WorldIconPath::new(input);

        // Then
        assert!(matches!(result, Err(DomainError::InvalidWorldIconPath(_))));
    }
}
