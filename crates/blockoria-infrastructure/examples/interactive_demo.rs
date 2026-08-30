// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! Interactive demo CLI for manual testing.
//!
//! Usage:
//!   cargo run --example interactive_demo --package blockoria-infrastructure
//!
//! Creates a persistent test environment in ./test_env/ so you can inspect files.

use blockoria_application::ports::BackupRepository;
use blockoria_application::use_cases::{
    create_backup::create_backup, delete_backup::delete_backup, list_backups::list_backups,
    list_worlds::list_worlds, restore_backup::restore_backup,
};
use blockoria_domain::{Backup, BackupPath, DomainError};
use blockoria_infrastructure::{FileBackupRepository, FileWorldRepository};
use std::io::{self, Write};
use std::path::PathBuf;

const TEST_ENV_DIR: &str = "./test_env";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Blockoria Interactive Demo ===\n");

    // Setup persistent test environment
    setup_test_env()?;

    let users_dir = PathBuf::from(TEST_ENV_DIR).join("Users");
    let backups_dir = PathBuf::from(TEST_ENV_DIR).join("backups");

    let world_repo = FileWorldRepository::with_path(&users_dir);
    let backup_repo = FileBackupRepository::new(&backups_dir);
    let backup_root = BackupPath::new(&backups_dir)?;

    loop {
        println!("\n{}", "=".repeat(50));
        println!("MENU");
        println!("{}", "=".repeat(50));
        println!("1. Listar mundos");
        println!("2. Criar backup");
        println!("3. Listar backups de um mundo");
        println!("4. Restaurar backup");
        println!("5. Deletar backup");
        println!("6. Mostrar estrutura de arquivos");
        println!("7. Limpar ambiente de teste");
        println!("0. Sair");
        print!("\nEscolha: ");
        io::stdout().flush()?;

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break, // EOF
            Ok(_) => {}
            Err(_) => break,
        }
        let choice = input.trim();

        if choice.is_empty() {
            continue;
        }

        match choice {
            "1" => list_worlds_cmd(&world_repo)?,
            "2" => create_backup_cmd(&world_repo, &backup_repo, &backup_root)?,
            "3" => list_backups_cmd(&backup_repo)?,
            "4" => restore_backup_cmd(&world_repo, &backup_repo)?,
            "5" => delete_backup_cmd(&backup_repo)?,
            "6" => show_file_structure()?,
            "7" => cleanup_test_env()?,
            "0" => {
                println!("\nSaindo... Ambiente mantido em ./{}", TEST_ENV_DIR);
                break;
            }
            _ => println!("Opção inválida!"),
        }
    }

    Ok(())
}

fn setup_test_env() -> Result<(), Box<dyn std::error::Error>> {
    let users_dir = PathBuf::from(TEST_ENV_DIR).join("Users");
    let backups_dir = PathBuf::from(TEST_ENV_DIR).join("backups");

    if users_dir.exists() {
        println!("Ambiente de teste já existe em ./{}/", TEST_ENV_DIR);
        return Ok(());
    }

    println!("Criando ambiente de teste em ./{}/...", TEST_ENV_DIR);
    std::fs::create_dir_all(&users_dir)?;
    std::fs::create_dir_all(&backups_dir)?;

    // Mundo Account 1
    let w1 = users_dir
        .join("111111111111111111")
        .join("games")
        .join("com.mojang")
        .join("minecraftWorlds")
        .join("aaaaaaaaaaa=");
    std::fs::create_dir_all(&w1)?;
    std::fs::write(w1.join("level.dat"), b"fake level.dat v1.21.0")?;
    std::fs::write(w1.join("levelname.txt"), b"Mundo Sobrevivencia")?;
    std::fs::write(w1.join("world_icon.jpeg"), b"fake icon")?;
    std::fs::create_dir_all(w1.join("db"))?;
    std::fs::write(w1.join("db").join("player.dat"), b"player data")?;
    std::fs::create_dir_all(w1.join("region"))?;
    std::fs::write(w1.join("region").join("r.0.0.mca"), b"chunk data")?;

    // Mundo Account 2
    let w2 = users_dir
        .join("222222222222222222")
        .join("games")
        .join("com.mojang")
        .join("minecraftWorlds")
        .join("bbbbbbbbbbb=");
    std::fs::create_dir_all(&w2)?;
    std::fs::write(w2.join("level.dat"), b"fake level.dat v1.21.0")?;
    std::fs::write(w2.join("levelname.txt"), b"Creative Mode")?;
    std::fs::write(w2.join("world_icon.jpeg"), b"fake icon")?;

    // Mundo Shared
    let w3 = users_dir
        .join("Shared")
        .join("games")
        .join("com.mojang")
        .join("minecraftWorlds")
        .join("ccccccccccc=");
    std::fs::create_dir_all(&w3)?;
    std::fs::write(w3.join("level.dat"), b"fake level.dat v1.21.0")?;
    std::fs::write(w3.join("levelname.txt"), b"Servidor Compartilhado")?;
    std::fs::write(w3.join("world_icon.jpeg"), b"fake icon")?;

    println!("✓ 3 mundos criados (2 Account + 1 Shared)");
    println!("✓ Pronto para testar!\n");
    Ok(())
}

fn cleanup_test_env() -> Result<(), Box<dyn std::error::Error>> {
    let path = PathBuf::from(TEST_ENV_DIR);
    if path.exists() {
        std::fs::remove_dir_all(&path)?;
        println!("✓ Ambiente de teste removido: ./{}/", TEST_ENV_DIR);
    } else {
        println!("Ambiente já não existe.");
    }
    Ok(())
}

fn show_file_structure() -> Result<(), Box<dyn std::error::Error>> {
    let path = PathBuf::from(TEST_ENV_DIR);
    if !path.exists() {
        println!("Ambiente não existe. Rode a opção 1 primeiro.");
        return Ok(());
    }

    println!("\n=== Estrutura de arquivos ===");
    print_tree(&path, 0)?;
    Ok(())
}

fn print_tree(dir: &PathBuf, depth: usize) -> Result<(), Box<dyn std::error::Error>> {
    let indent = "  ".repeat(depth);
    let entries = std::fs::read_dir(dir)?;
    let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
    entries.sort_by_key(|e| e.file_name());

    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == entries.len() - 1;
        let prefix = if is_last { "└── " } else { "├── " };
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        if entry.file_type()?.is_dir() {
            println!("{}{}{}/", indent, prefix, name);
            let next_indent = if is_last { "    " } else { "│   " };
            print_tree_with_indent(&entry.path(), depth + 1, next_indent)?;
        } else {
            let size = entry.metadata()?.len();
            println!("{}{}{} ({} bytes)", indent, prefix, name, size);
        }
    }
    Ok(())
}

fn print_tree_with_indent(
    dir: &PathBuf,
    depth: usize,
    indent: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let entries = std::fs::read_dir(dir)?;
    let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
    entries.sort_by_key(|e| e.file_name());

    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == entries.len() - 1;
        let prefix = if is_last { "└── " } else { "├── " };
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        if entry.file_type()?.is_dir() {
            println!("{}{}{}/", indent, prefix, name);
            let next_indent = if is_last { "    " } else { "│   " };
            print_tree_with_indent(&entry.path(), depth + 1, next_indent)?;
        } else {
            let size = entry.metadata()?.len();
            println!("{}{}{} ({} bytes)", indent, prefix, name, size);
        }
    }
    Ok(())
}

fn list_worlds_cmd(world_repo: &FileWorldRepository) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Mundos encontrados ---");
    let worlds = list_worlds(world_repo)?;

    if worlds.is_empty() {
        println!("  (nenhum mundo encontrado)");
        return Ok(());
    }

    for (i, w) in worlds.iter().enumerate() {
        let loc = if w.is_shared() { "Shared" } else { "Account" };
        let acc = w.account_id().map(|a| a.as_str()).unwrap_or("-");
        println!("  [{}] {} ", i + 1, w.level_name().as_str());
        println!(
            "      Folder: {} | Location: {} | Account: {}",
            w.folder_name().as_str(),
            loc,
            acc
        );
        println!("      Path: {}", w.path().as_path().display());
    }
    Ok(())
}

fn select_world(
    world_repo: &FileWorldRepository,
) -> Result<
    Option<(
        blockoria_domain::WorldFolderName,
        blockoria_domain::WorldLocation,
        blockoria_domain::AccountId,
    )>,
    Box<dyn std::error::Error>,
> {
    let worlds = list_worlds(world_repo)?;
    if worlds.is_empty() {
        println!("  Nenhum mundo disponível.");
        return Ok(None);
    }

    print!("\nEscolha o número do mundo (0 para cancelar): ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let idx: usize = input.trim().parse().unwrap_or(0);

    if idx == 0 || idx > worlds.len() {
        return Ok(None);
    }

    let world = &worlds[idx - 1];
    let folder = world.folder_name().clone();
    let location = world.location().clone();
    let account = world
        .account_id()
        .cloned()
        .unwrap_or_else(|| blockoria_domain::AccountId::new("shared").unwrap());

    Ok(Some((folder, location, account)))
}

fn create_backup_cmd(
    world_repo: &FileWorldRepository,
    backup_repo: &FileBackupRepository,
    backup_root: &BackupPath,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Criar Backup ---");
    let Some((folder, location, account_id)) = select_world(world_repo)? else {
        return Ok(());
    };

    // Find the world
    let worlds = list_worlds(world_repo)?;
    let world = worlds
        .iter()
        .find(|w| w.folder_name().as_str() == folder.as_str())
        .unwrap();

    println!("\nCriando backup para '{}'...", world.level_name().as_str());
    let backup = create_backup(world, backup_root)?;

    // Move to correct repo structure: backup_root/account_id/folder/timestamp
    let correct_dir = backup_root
        .as_path()
        .join(account_id.as_str())
        .join(folder.as_str().replace('=', "_"))
        .join(backup.created_at().to_filename_safe());

    std::fs::create_dir_all(correct_dir.parent().unwrap())?;
    if backup.backup_path().as_path().exists() {
        std::fs::rename(backup.backup_path().as_path(), &correct_dir)?;
    }

    let correct_backup = Backup::new(
        folder.clone(),
        account_id.clone(),
        world.version().clone(),
        backup.created_at().clone(),
        BackupPath::new(&correct_dir)?,
    );

    backup_repo.save(&correct_backup)?;

    println!("✓ Backup criado!");
    println!("  Local: {}", correct_dir.display());
    println!(
        "  Timestamp: {}",
        correct_backup.created_at().to_filename_safe()
    );
    Ok(())
}

fn list_backups_cmd(backup_repo: &FileBackupRepository) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Listar Backups ---");
    let worlds = list_worlds(&FileWorldRepository::with_path(
        PathBuf::from(TEST_ENV_DIR).join("Users"),
    ))?;
    if worlds.is_empty() {
        println!("  Nenhum mundo.");
        return Ok(());
    }

    print!("\nEscolha o mundo (número): ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let idx: usize = input.trim().parse().unwrap_or(0);

    if idx == 0 || idx > worlds.len() {
        return Ok(());
    }

    let world = &worlds[idx - 1];
    let folder = world.folder_name().clone();
    let location = world.location().clone();

    let backups = list_backups(backup_repo, &folder, &location)?;

    if backups.is_empty() {
        println!("  Nenhum backup para este mundo.");
    } else {
        println!("\n  Backups de '{}':", world.level_name().as_str());
        for (i, b) in backups.iter().enumerate() {
            println!(
                "    [{}] {} | v{:?} | {}",
                i + 1,
                b.created_at().to_filename_safe(),
                b.world_version().as_array(),
                b.backup_path().as_path().display()
            );
        }
    }
    Ok(())
}

fn restore_backup_cmd(
    world_repo: &FileWorldRepository,
    backup_repo: &FileBackupRepository,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Restaurar Backup ---");
    let worlds = list_worlds(world_repo)?;
    if worlds.is_empty() {
        println!("  Nenhum mundo.");
        return Ok(());
    }

    print!("\nEscolha o mundo (número): ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let idx: usize = input.trim().parse().unwrap_or(0);

    if idx == 0 || idx > worlds.len() {
        return Ok(());
    }

    let world = &worlds[idx - 1];
    let folder = world.folder_name().clone();
    let location = world.location().clone();

    let backups = list_backups(backup_repo, &folder, &location)?;
    if backups.is_empty() {
        println!("  Nenhum backup para restaurar.");
        return Ok(());
    }

    println!("\n  Backups disponíveis:");
    for (i, b) in backups.iter().enumerate() {
        println!("    [{}] {}", i + 1, b.created_at().to_filename_safe());
    }

    print!("\nEscolha o backup (número): ");
    io::stdout().flush()?;
    input.clear();
    io::stdin().read_line(&mut input)?;
    let bidx: usize = input.trim().parse().unwrap_or(0);

    if bidx == 0 || bidx > backups.len() {
        return Ok(());
    }

    let backup = &backups[bidx - 1];
    let restore_target = PathBuf::from(TEST_ENV_DIR)
        .join("restored")
        .join(&backup.created_at().to_filename_safe());

    println!("\nRestaurando para: {}", restore_target.display());
    // Note: restore_backup expects (backup_repo, world_repo, folder, location, backup_path)
    restore_backup(
        backup_repo,
        world_repo,
        &folder,
        &location,
        backup.backup_path(),
    )?;
    println!("✓ Restaurado! Verifique em: {}", restore_target.display());
    Ok(())
}

fn delete_backup_cmd(backup_repo: &FileBackupRepository) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Deletar Backup ---");
    let worlds = list_worlds(&FileWorldRepository::with_path(
        PathBuf::from(TEST_ENV_DIR).join("Users"),
    ))?;
    if worlds.is_empty() {
        println!("  Nenhum mundo.");
        return Ok(());
    }

    print!("\nEscolha o mundo (número): ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let idx: usize = input.trim().parse().unwrap_or(0);

    if idx == 0 || idx > worlds.len() {
        return Ok(());
    }

    let world = &worlds[idx - 1];
    let folder = world.folder_name().clone();
    let location = world.location().clone();

    let backups = list_backups(backup_repo, &folder, &location)?;
    if backups.is_empty() {
        println!("  Nenhum backup para deletar.");
        return Ok(());
    }

    println!("\n  Backups:");
    for (i, b) in backups.iter().enumerate() {
        println!("    [{}] {}", i + 1, b.created_at().to_filename_safe());
    }

    print!("\nEscolha o backup para DELETAR (número): ");
    io::stdout().flush()?;
    input.clear();
    io::stdin().read_line(&mut input)?;
    let bidx: usize = input.trim().parse().unwrap_or(0);

    if bidx == 0 || bidx > backups.len() {
        return Ok(());
    }

    let backup = &backups[bidx - 1];
    print!(
        "Confirmar deleção de '{}'? (s/N): ",
        backup.created_at().to_filename_safe()
    );
    io::stdout().flush()?;
    input.clear();
    io::stdin().read_line(&mut input)?;

    if input.trim().to_lowercase() == "s" {
        delete_backup(backup_repo, backup.backup_path())?;
        println!("✓ Backup deletado!");
    } else {
        println!("Cancelado.");
    }
    Ok(())
}
