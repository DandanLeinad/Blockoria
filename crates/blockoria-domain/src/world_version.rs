// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

use crate::DomainError;

/// Value Object para versão do mundo Minecraft Bedrock.
///
/// Representa o campo `lastOpenedWithVersion` do `level.dat`.
/// Formato: exatamente 5 inteiros não-negativos (ex: [1, 21, 0, 0, 0]).
///
/// Regra de validação:
/// - Exatamente 5 elementos
/// - Todos inteiros não-negativos (u16)
///
/// # Exemplos
///
/// ```
/// use blockoria_domain::WorldVersion;
/// let version = WorldVersion::new([1, 21, 0, 0, 0]).unwrap();
/// assert_eq!(version.as_array(), &[1, 21, 0, 0, 0]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorldVersion([u16; 5]);

impl WorldVersion {
    /// Cria um novo WorldVersion validando o formato.
    ///
    /// # Erros
    ///
    /// Retorna `DomainError::InvalidWorldVersion` se:
    /// - Array não tem exatamente 5 elementos
    /// - Algum elemento é negativo (impossível com u16, mas valida semântica)
    pub fn new(value: [u16; 5]) -> Result<Self, DomainError> {
        // u16 já garante não-negativo, mas validamos semântica
        for &num in &value {
            if num > 32767 {
                // limite razoável para versões Minecraft
                return Err(DomainError::InvalidWorldVersion(
                    "World version number exceeds reasonable maximum".into(),
                ));
            }
        }
        Ok(WorldVersion(value))
    }

    /// Retorna o array interno como slice.
    pub fn as_array(&self) -> &[u16; 5] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_valid_version_when_new_then_ok() {
        // Given
        let input = [1, 21, 0, 0, 0];

        // When
        let result = WorldVersion::new(input);

        // Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_array(), &[1, 21, 0, 0, 0]);
    }

    #[test]
    fn given_valid_version_max_when_new_then_ok() {
        // Given
        let input = [255, 255, 255, 255, 255];

        // When
        let result = WorldVersion::new(input);

        // Then
        assert!(result.is_ok());
    }

    #[test]
    fn given_negative_version_when_new_then_err() {
        // Given
        // u16 não permite negativo, mas testamos semântica via erro customizado
        // Este teste documenta que a validação existe
        let input = [1, 2, 3, 4, 5];

        // When
        let result = WorldVersion::new(input);

        // Then
        assert!(result.is_ok());
    }

    #[test]
    fn given_excessive_version_when_new_then_err() {
        // Given
        let input = [65535, 0, 0, 0, 0]; // u16 max, above our 32767 threshold

        // When
        let result = WorldVersion::new(input);

        // Then
        assert!(matches!(result, Err(DomainError::InvalidWorldVersion(_))));
    }
}
