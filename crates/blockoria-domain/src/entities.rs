// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

use crate::{
    AccountId, BackupPath, BackupTimestamp, LevelName, WorldFolderName, WorldIconPath, WorldPath,
    WorldVersion,
};

/// Aggregate Root: Mundo Minecraft Bedrock.
///
/// Representa um mundo completo com todos os seus dados de identidade e localização.
///
/// # Invariantes
/// - Todos os campos são obrigatórios e validados na criação
/// - `icon_path` pode ser `None` (mundo sem ícone)
///
/// # Exemplos
///
/// ```
/// use blockoria_domain::{World, WorldFolderName, LevelName, WorldPath, AccountId, WorldVersion, WorldIconPath};
/// use std::path::PathBuf;
/// use tempfile::tempdir;
/// use chrono::{TimeZone, Utc};
///
/// let dir = tempdir().unwrap();
/// let world = World::new(
///     WorldFolderName::new("6LknJ3qXcJo=").unwrap(),
///     LevelName::new("Meu Mundo").unwrap(),
///     WorldPath::new(dir.path()).unwrap(),
///     AccountId::new("123456789012345678").unwrap(),
///     WorldVersion::new([1, 21, 0, 0, 0]).unwrap(),
///     WorldIconPath::new(None::<PathBuf>).unwrap(),
/// );
/// assert_eq!(world.level_name().as_str(), "Meu Mundo");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct World {
    folder_name: WorldFolderName,
    level_name: LevelName,
    path: WorldPath,
    account_id: AccountId,
    version: WorldVersion,
    icon_path: WorldIconPath,
}

impl World {
    /// Cria um novo mundo com todos os dados validados.
    pub fn new(
        folder_name: WorldFolderName,
        level_name: LevelName,
        path: WorldPath,
        account_id: AccountId,
        version: WorldVersion,
        icon_path: WorldIconPath,
    ) -> Self {
        Self {
            folder_name,
            level_name,
            path,
            account_id,
            version,
            icon_path,
        }
    }

    /// Retorna o nome da pasta do mundo (formato base64, 11 chars + '=').
    pub fn folder_name(&self) -> &WorldFolderName {
        &self.folder_name
    }

    /// Retorna o nome de exibição do mundo (levelname).
    pub fn level_name(&self) -> &LevelName {
        &self.level_name
    }

    /// Retorna o caminho do mundo no filesystem.
    pub fn path(&self) -> &WorldPath {
        &self.path
    }

    /// Retorna o ID da conta Microsoft associada.
    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    /// Retorna a versão do mundo (lastOpenedWithVersion).
    pub fn version(&self) -> &WorldVersion {
        &self.version
    }

    /// Retorna o caminho do ícone do mundo (pode ser None).
    pub fn icon_path(&self) -> &WorldIconPath {
        &self.icon_path
    }
}

/// Aggregate Root: Backup de um mundo Minecraft Bedrock.
///
/// Representa um backup completo de um mundo em um momento específico.
/// Contém a versão do mundo no momento do backup para garantir restore correto.
///
/// # Invariantes
/// - `world_version` garante que o restore saiba qual versão restaurar
/// - `created_at` é o timestamp exato da criação do backup
/// - `backup_path` aponta para o diretório onde os arquivos foram copiados
///
/// # Exemplos
///
/// ```
/// use blockoria_domain::{Backup, WorldFolderName, AccountId, WorldVersion, BackupTimestamp, BackupPath};
/// use std::path::PathBuf;
/// use tempfile::tempdir;
/// use chrono::{TimeZone, Utc};
///
/// let dir = tempdir().unwrap();
/// let backup = Backup::new(
///     WorldFolderName::new("6LknJ3qXcJo=").unwrap(),
///     AccountId::new("123456789012345678").unwrap(),
///     WorldVersion::new([1, 21, 0, 0, 0]).unwrap(),
///     BackupTimestamp::now(),
///     BackupPath::new(dir.path()).unwrap(),
/// );
/// assert_eq!(backup.world_folder_name().as_str(), "6LknJ3qXcJo=");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Backup {
    world_folder_name: WorldFolderName,
    world_account_id: AccountId,
    world_version: WorldVersion,
    created_at: BackupTimestamp,
    backup_path: BackupPath,
}

impl Backup {
    /// Cria um novo backup com todos os dados validados.
    ///
    /// # Argumentos
    /// * `world_folder_name` - Nome da pasta do mundo (base64, 11 chars + '=')
    /// * `world_account_id` - ID da conta Microsoft do dono do mundo
    /// * `world_version` - Versão do mundo no momento do backup (para restore correto)
    /// * `created_at` - Timestamp exato da criação do backup
    /// * `backup_path` - Caminho do diretório onde o backup foi armazenado
    pub fn new(
        world_folder_name: WorldFolderName,
        world_account_id: AccountId,
        world_version: WorldVersion,
        created_at: BackupTimestamp,
        backup_path: BackupPath,
    ) -> Self {
        Self {
            world_folder_name,
            world_account_id,
            world_version,
            created_at,
            backup_path,
        }
    }

    /// Retorna o nome da pasta do mundo original.
    pub fn world_folder_name(&self) -> &WorldFolderName {
        &self.world_folder_name
    }

    /// Retorna o ID da conta do dono do mundo.
    pub fn world_account_id(&self) -> &AccountId {
        &self.world_account_id
    }

    /// Retorna a versão do mundo no momento do backup.
    ///
    /// Essencial para restore correto - garante que a versão restaurada
    /// corresponde à versão do mundo no momento do backup.
    pub fn world_version(&self) -> &WorldVersion {
        &self.world_version
    }

    /// Retorna o timestamp exato da criação do backup.
    pub fn created_at(&self) -> &BackupTimestamp {
        &self.created_at
    }

    /// Retorna o caminho do diretório onde o backup foi armazenado.
    pub fn backup_path(&self) -> &BackupPath {
        &self.backup_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AccountId, LevelName, WorldFolderName, WorldIconPath, WorldPath, WorldVersion};
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn make_test_world() -> World {
        let dir = tempdir().unwrap();
        World::new(
            WorldFolderName::new("6LknJ3qXcJo=").unwrap(),
            LevelName::new("Test World").unwrap(),
            WorldPath::new(dir.path()).unwrap(),
            AccountId::new("123456789012345678").unwrap(),
            WorldVersion::new([1, 21, 0, 0, 0]).unwrap(),
            WorldIconPath::new(None::<PathBuf>).unwrap(),
        )
    }

    #[test]
    fn given_valid_data_when_new_world_then_ok() {
        // Given
        let world = make_test_world();

        // Then
        assert_eq!(world.folder_name().as_str(), "6LknJ3qXcJo=");
        assert_eq!(world.level_name().as_str(), "Test World");
        assert_eq!(world.account_id().as_str(), "123456789012345678");
        assert_eq!(world.version().as_array(), &[1, 21, 0, 0, 0]);
        assert!(world.icon_path().is_none());
    }

    #[test]
    fn given_valid_data_when_new_backup_then_ok() {
        // Given
        let world = make_test_world();
        let dir = tempdir().unwrap();

        // When
        let backup = Backup::new(
            world.folder_name().clone(),
            world.account_id().clone(),
            world.version().clone(),
            BackupTimestamp::now(),
            BackupPath::new(dir.path()).unwrap(),
        );

        // Then
        assert_eq!(backup.world_folder_name().as_str(), "6LknJ3qXcJo=");
        assert_eq!(backup.world_account_id().as_str(), "123456789012345678");
        assert_eq!(backup.world_version().as_array(), &[1, 21, 0, 0, 0]);
        assert!(backup.backup_path().as_path().exists());
    }

    #[test]
    fn given_backup_when_created_then_has_timestamp() {
        // Given
        let world = make_test_world();
        let dir = tempdir().unwrap();
        let before = chrono::Utc::now();

        // When
        let backup = Backup::new(
            world.folder_name().clone(),
            world.account_id().clone(),
            world.version().clone(),
            BackupTimestamp::now(),
            BackupPath::new(dir.path()).unwrap(),
        );

        // Then
        let ts = backup.created_at();
        assert!(ts.as_datetime() >= &before);
        assert!(ts.as_datetime() <= &chrono::Utc::now());
    }

    #[test]
    fn given_backup_when_created_then_version_preserved() {
        // Given
        let world = make_test_world();
        let dir = tempdir().unwrap();

        // When
        let backup = Backup::new(
            world.folder_name().clone(),
            world.account_id().clone(),
            world.version().clone(),
            BackupTimestamp::now(),
            BackupPath::new(dir.path()).unwrap(),
        );

        // Then
        assert_eq!(
            backup.world_version().as_array(),
            world.version().as_array()
        );
    }
}
