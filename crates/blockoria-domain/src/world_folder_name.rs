// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

use crate::DomainError;

/// Value Object para nome de pasta do mundo Minecraft Bedrock.
///
/// Regras de validação (baseado no formato base64 do Minecraft):
/// - Exatamente 12 caracteres
/// - Termina com '=' (padding base64)
/// - Não vazio, não apenas whitespace
///
/// # Exemplos
///
/// ```
/// use blockoria_domain::WorldFolderName;
/// let name = WorldFolderName::new("6LknJ3qXcJo=").unwrap();
/// assert_eq!(name.as_str(), "6LknJ3qXcJo=");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorldFolderName(String);

impl WorldFolderName {
    /// Cria um novo WorldFolderName validando o formato.
    ///
    /// # Erros
    ///
    /// Retorna `DomainError::InvalidWorldFolderName` se:
    /// - String vazia ou apenas whitespace
    /// - Tamanho diferente de 12 caracteres
    /// - Não termina com '='
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(DomainError::InvalidWorldFolderName(
                "World folder name cannot be empty or whitespace".into(),
            ));
        }
        if value.len() != 12 {
            return Err(DomainError::InvalidWorldFolderName(
                "World folder name must be exactly 12 characters long".into(),
            ));
        }
        if !value.ends_with('=') {
            return Err(DomainError::InvalidWorldFolderName(
                "World folder name must end with '='".into(),
            ));
        }
        Ok(WorldFolderName(value))
    }

    /// Retorna a string interna como slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn given_valid_12chars_ends_eq_when_new_then_ok() {
        // Given
        let input = "6LknJ3qXcJo=";

        // When
        let result = super::WorldFolderName::new(input);

        // Then
        assert!(result.is_ok());
    }

    #[test]
    fn given_empty_when_new_then_err_empty() {
        // Given
        let input = "";

        // When
        let result = super::WorldFolderName::new(input);

        // Then
        assert!(result.is_err());
    }

    #[test]
    fn given_whitespace_only_when_new_then_err_empty() {
        // Given
        let input = "   ";

        // When
        let result = super::WorldFolderName::new(input);

        // Then
        assert!(result.is_err());
    }

    #[test]
    fn given_11chars_when_new_then_err_length() {
        // Given
        let input = "invalid_11";

        // When
        let result = super::WorldFolderName::new(input);

        // Then
        assert!(result.is_err());
    }

    #[test]
    fn given_13chars_when_new_then_err_length() {
        // Given
        let input = "invalid_13chars=";

        // When
        let result = super::WorldFolderName::new(input);

        // Then
        assert!(result.is_err());
    }

    #[test]
    fn given_no_equals_suffix_when_new_then_err_format() {
        // Given
        let input = "invalid_12chars";

        // When
        let result = super::WorldFolderName::new(input);

        // Then
        assert!(result.is_err());
    }

    #[test]
    fn given_valid_but_no_eq_when_new_then_err_format() {
        // Given
        let input = "valid_12chars";

        // When
        let result = super::WorldFolderName::new(input);

        // Then
        assert!(result.is_err());
    }
}
