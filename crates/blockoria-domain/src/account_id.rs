// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

use crate::DomainError;

/// Value Object para ID da conta Microsoft do jogador.
///
/// Identificador único da conta Microsoft associada ao mundo.
/// Lido do arquivo `level.dat` ou configuração do Minecraft Bedrock.
///
/// Regra de validação:
/// - Não pode ser vazio ou apenas whitespace
///
/// # Exemplos
///
/// ```
/// use blockoria_domain::AccountId;
/// let id = AccountId::new("123456789012345678").unwrap();
/// assert_eq!(id.as_str(), "123456789012345678");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountId(String);

impl AccountId {
    /// Cria um novo AccountId validando o formato.
    ///
    /// # Erros
    ///
    /// Retorna `DomainError::InvalidAccountId` se:
    /// - String vazia
    /// - Apenas whitespace (espaços, tabs, newlines)
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(DomainError::InvalidAccountId(
                "Account ID cannot be empty or whitespace".into(),
            ));
        }
        Ok(AccountId(value))
    }

    /// Retorna a string interna como slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_valid_id_when_new_then_ok() {
        // Given
        let input = "123456789012345678";

        // When
        let result = AccountId::new(input);

        // Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "123456789012345678");
    }

    #[test]
    fn given_empty_when_new_then_err() {
        // Given
        let input = "";

        // When
        let result = AccountId::new(input);

        // Then
        assert!(matches!(result, Err(DomainError::InvalidAccountId(_))));
    }

    #[test]
    fn given_whitespace_only_when_new_then_err() {
        // Given
        let input = "   ";

        // When
        let result = AccountId::new(input);

        // Then
        assert!(matches!(result, Err(DomainError::InvalidAccountId(_))));
    }

    #[test]
    fn given_tabs_newlines_when_new_then_err() {
        // Given
        let input = "\t\n\r";

        // When
        let result = AccountId::new(input);

        // Then
        assert!(matches!(result, Err(DomainError::InvalidAccountId(_))));
    }
}
