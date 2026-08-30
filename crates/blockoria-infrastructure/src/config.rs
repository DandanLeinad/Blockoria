// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! Configuration for blockoria-infrastructure.
//!
//! Loads configuration from `%APPDATA%\Blockoria\config.toml`.
//! Falls back to sensible defaults if config file doesn't exist.

use blockoria_domain::DomainError;
use dirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Application configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Root directory for backups.
    #[serde(default = "default_backup_root")]
    pub backup_root: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            backup_root: default_backup_root(),
        }
    }
}

fn default_backup_root() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("Blockoria")
        .join("backups")
}

impl Config {
    /// Loads configuration from the default config file location.
    ///
    /// Config file: `%APPDATA%\Blockoria\config.toml`
    pub fn load() -> Result<Self, DomainError> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            // Create default config file
            let default = Self::default();
            default.save()?;
            return Ok(default);
        }

        let content = fs::read_to_string(&config_path)
            .map_err(|e| DomainError::InvalidBackupPath(format!("Failed to read config: {}", e)))?;

        let config: Config = toml::from_str(&content).map_err(|e| {
            DomainError::InvalidBackupPath(format!("Failed to parse config: {}", e))
        })?;

        Ok(config)
    }

    /// Saves configuration to the default config file location.
    pub fn save(&self) -> Result<(), DomainError> {
        let config_path = Self::config_path()?;

        // Ensure parent directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                DomainError::InvalidBackupPath(format!("Failed to create config dir: {}", e))
            })?;
        }

        let content = toml::to_string_pretty(self).map_err(|e| {
            DomainError::InvalidBackupPath(format!("Failed to serialize config: {}", e))
        })?;

        fs::write(&config_path, content).map_err(|e| {
            DomainError::InvalidBackupPath(format!("Failed to write config: {}", e))
        })?;

        Ok(())
    }

    /// Returns the path to the config file.
    fn config_path() -> Result<PathBuf, DomainError> {
        Ok(dirs::data_dir()
            .ok_or_else(|| {
                DomainError::InvalidBackupPath("Could not find APPDATA directory".into())
            })?
            .join("Blockoria")
            .join("config.toml"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.backup_root.ends_with("Blockoria/backups"));
    }
}
