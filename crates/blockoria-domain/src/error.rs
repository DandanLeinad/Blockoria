// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

use std::fmt;

/// Erros de validação do domínio Blockoria.
///
/// Cada variante carrega uma mensagem descritiva do problema de validação.
/// Use `DomainError::Variante("detalhe")` para criar instâncias.
///
/// # Exemplos
///
/// ```
/// use blockoria_domain::DomainError;
/// let err = DomainError::InvalidWorldFolderName("empty".into());
/// assert_eq!(err.to_string(), "InvalidWorldFolderName: empty");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainError {
    /// Pasta do mundo inválida.
    ///
    /// Regras: exatamente 12 caracteres, termina com '=', não apenas whitespace.
    /// Ex: `InvalidWorldFolderName("empty")`, `InvalidWorldFolderName("11 chars")`
    InvalidWorldFolderName(String),

    /// Caminho do mundo inválido.
    ///
    /// Regras: não vazio, deve existir no filesystem, deve ser diretório.
    InvalidWorldPath(String),

    /// Caminho do ícone do mundo inválido.
    ///
    /// Regras: não vazio, deve existir, deve ser arquivo (world_icon.jpeg).
    InvalidWorldIconPath(String),

    /// Nome do mundo (levelname) inválido.
    ///
    /// Regra: não pode ser vazio ou apenas whitespace.
    InvalidLevelName(String),

    /// ID da conta Microsoft inválido.
    ///
    /// Regra: não pode ser vazio ou apenas whitespace.
    InvalidAccountId(String),

    /// Versão do mundo inválida.
    ///
    /// Regras: deve ser lista de exatamente 5 inteiros não-negativos.
    InvalidWorldVersion(String),
}

/// Implementação Display para mensagens legíveis em logs/UX.
impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::InvalidWorldFolderName(msg) => {
                write!(f, "InvalidWorldFolderName: {}", msg)
            }
            DomainError::InvalidWorldPath(msg) => write!(f, "InvalidWorldPath: {}", msg),
            DomainError::InvalidWorldIconPath(msg) => {
                write!(f, "InvalidWorldIconPath: {}", msg)
            }
            DomainError::InvalidAccountId(msg) => write!(f, "InvalidAccountId: {}", msg),
            DomainError::InvalidWorldVersion(msg) => write!(f, "InvalidWorldVersion: {}", msg),
            DomainError::InvalidLevelName(msg) => write!(f, "InvalidLevelName: {}", msg),
        }
    }
}

impl std::error::Error for DomainError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_formats_variant_message() {
        // Given
        let err = DomainError::InvalidWorldFolderName("empty".into());

        // When
        let display = err.to_string();

        // Then
        assert_eq!(display, "InvalidWorldFolderName: empty");
    }

    #[test]
    fn error_trait_implemented() {
        // Given
        let err = DomainError::InvalidWorldPath("not found".into());

        // When
        let _: &dyn std::error::Error = &err;

        // Then (compiles if Error trait implemented)
    }

    #[test]
    fn partial_eq_works_for_assertions() {
        // Given
        let a = DomainError::InvalidLevelName("x".into());
        let b = DomainError::InvalidLevelName("x".into());

        // When
        let equal = a == b;

        // Then
        assert!(equal);
    }

    #[test]
    fn clone_works() {
        // Given
        let err = DomainError::InvalidWorldVersion("bad".into());

        // When
        let cloned = err.clone();

        // Then
        assert_eq!(err, cloned);
    }

    #[test]
    fn world_icon_path_error_works() {
        // Given
        let err = DomainError::InvalidWorldIconPath("not a file".into());

        // When
        let display = err.to_string();

        // Then
        assert_eq!(display, "InvalidWorldIconPath: not a file");
    }

    #[test]
    fn account_id_error_works() {
        // Given
        let err = DomainError::InvalidAccountId("empty".into());

        // When
        let display = err.to_string();

        // Then
        assert_eq!(display, "InvalidAccountId: empty");
    }

    #[test]
    fn world_version_error_works() {
        // Given
        let err = DomainError::InvalidWorldVersion("not 5 ints".into());

        // When
        let display = err.to_string();

        // Then
        assert_eq!(display, "InvalidWorldVersion: not 5 ints");
    }
}
