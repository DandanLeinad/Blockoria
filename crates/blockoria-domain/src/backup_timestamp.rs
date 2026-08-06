// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

use crate::DomainError;
use chrono::{DateTime, Utc};

/// Value Object para timestamp de criação do backup.
///
/// Armazena a data/hora UTC exata da criação do backup.
/// Usa `chrono::DateTime<Utc>` internamente para precisão e serialização.
///
/// Regra de validação:
/// - Timestamp não pode ser anterior a 1970 (Unix epoch)
///
/// # Exemplos
///
/// ```
/// use blockoria_domain::BackupTimestamp;
/// use chrono::{TimeZone, Utc, Datelike};
///
/// let ts = BackupTimestamp::new(Utc.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap()).unwrap();
/// assert_eq!(ts.as_datetime().year(), 2024);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupTimestamp(DateTime<Utc>);

impl BackupTimestamp {
    /// Cria um novo BackupTimestamp validando o timestamp.
    ///
    /// # Erros
    ///
    /// Retorna `DomainError::InvalidBackupTimestamp` se:
    /// - Timestamp anterior a 1970-01-01 00:00:00 UTC
    pub fn new(value: DateTime<Utc>) -> Result<Self, DomainError> {
        if value.timestamp() < 0 {
            return Err(DomainError::InvalidBackupTimestamp(
                "Backup timestamp cannot be before Unix epoch (1970-01-01)".into(),
            ));
        }
        Ok(BackupTimestamp(value))
    }

    /// Cria timestamp para o momento atual (now).
    pub fn now() -> Self {
        // unwrap seguro: now() sempre >= epoch
        BackupTimestamp::new(Utc::now()).expect("current time is always valid")
    }

    /// Retorna o `DateTime<Utc>` interno.
    pub fn as_datetime(&self) -> &DateTime<Utc> {
        &self.0
    }

    /// Formata como string ISO 8601 (ex: "2024-01-15T10:30:00Z").
    pub fn to_iso_string(&self) -> String {
        self.0.to_rfc3339()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn given_valid_utc_datetime_when_new_then_ok() {
        // Given
        let input = Utc.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap();

        // When
        let result = BackupTimestamp::new(input);

        // Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_datetime(), &input);
    }

    #[test]
    fn given_epoch_when_new_then_ok() {
        // Given
        let input = Utc.timestamp_opt(0, 0).single().unwrap();

        // When
        let result = BackupTimestamp::new(input);

        // Then
        assert!(result.is_ok());
    }

    #[test]
    fn given_before_epoch_when_new_then_err() {
        // Given
        let input = Utc.timestamp_opt(-1, 0).single().unwrap();

        // When
        let result = BackupTimestamp::new(input);

        // Then
        assert!(matches!(
            result,
            Err(DomainError::InvalidBackupTimestamp(_))
        ));
    }

    #[test]
    fn given_now_when_created_then_valid() {
        // When
        let before = chrono::Utc::now();
        let ts = BackupTimestamp::now();
        let after = chrono::Utc::now();

        // Then - timestamp should be between before and after (or equal)
        assert!(ts.as_datetime() >= &before);
        assert!(ts.as_datetime() <= &after);
    }

    #[test]
    fn given_timestamp_when_formatted_then_iso_string() {
        // Given
        let dt = Utc.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap();
        let ts = BackupTimestamp::new(dt).unwrap();

        // When
        let formatted = ts.to_iso_string();

        // Then
        assert_eq!(formatted, "2024-01-15T10:30:00+00:00");
    }
}
