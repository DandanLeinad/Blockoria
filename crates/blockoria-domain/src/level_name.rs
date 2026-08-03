// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

use crate::DomainError;

/// Value Object para o nome de exibição do mundo Minecraft Bedrock.
///
/// Este nome é lido do arquivo `levelname.txt` dentro da pasta do mundo.
/// É o nome que o jogador vê no menu de seleção de mundos.
///
/// Regra de validação:
/// - Não pode ser vazio ou apenas whitespace
///
/// # Exemplos
///
/// ```
/// use blockoria_domain::LevelName;
/// let name = LevelName::new("My World").unwrap();
/// assert_eq!(name.as_str(), "My World");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LevelName(String);

impl LevelName {
    /// Cria um novo LevelName validando o formato.
    ///
    /// # Erros
    ///
    /// Retorna `DomainError::InvalidLevelName` se:
    /// - String vazia
    /// - Apenas whitespace (espaços, tabs, newlines)
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(DomainError::InvalidLevelName(
                "Level name cannot be empty or whitespace".into(),
            ));
        }
        Ok(LevelName(value))
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
    fn given_valid_name_when_new_then_ok() {
        let input = "My World";
        let result = LevelName::new(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "My World");
    }

    #[test]
    fn given_empty_when_new_then_err() {
        let result = LevelName::new("");
        assert!(matches!(result, Err(DomainError::InvalidLevelName(_))));
    }

    #[test]
    fn given_whitespace_only_when_new_then_err() {
        let result = LevelName::new("   ");
        assert!(matches!(result, Err(DomainError::InvalidLevelName(_))));
    }

    #[test]
    fn given_tabs_newlines_when_new_then_err() {
        let result = LevelName::new("\t\n\r");
        assert!(matches!(result, Err(DomainError::InvalidLevelName(_))));
    }
}
