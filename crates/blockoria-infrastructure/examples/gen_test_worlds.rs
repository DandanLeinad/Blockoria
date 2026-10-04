// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! Generates minimal valid level.dat files for E2E test fixtures with full
//! Minecraft Bedrock directory structure.
//!
//! Run with: `cargo run --example gen_test_worlds`
//! Output: `../../app/e2e/fixtures/worlds/` (for local dev)
//!         or uses `BLOCKORIA_TEST_FIXTURES_DIR` env var for CI

use blockoria_infrastructure::nbt::{LevelDatHeader, NbtTagType, NbtValue};
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Default to temp dir to avoid polluting project
    // Set GEN_FIXTURES_TO_PROJECT=1 to write to ../../app/e2e/fixtures/worlds (local dev)
    let out_dir = if env::var("GEN_FIXTURES_TO_PROJECT").is_ok() {
        Path::new("../../app/e2e/fixtures/worlds").to_path_buf()
    } else if let Ok(dir) = env::var("BLOCKORIA_TEST_FIXTURES_DIR") {
        Path::new(&dir).to_path_buf()
    } else {
        std::env::temp_dir().join("blockoria-test-fixtures")
    };

    fs::create_dir_all(&out_dir)?;
    println!("Generating fixtures in: {}", out_dir.display());

    // Create the full Minecraft Bedrock Users directory structure
    let users_dir = out_dir.join("Users");
    fs::create_dir_all(&users_dir)?;

    // World 1: Xbox account world (version 1.21.0.0)
    create_account_world(
        &users_dir.join("111111111111111111"),
        "aaaaaaaaaaa=",
        "Test World 1",
        [1, 21, 0, 0, 0],
    )?;

    // World 2: Shared world (version 1.20.80.0)
    create_shared_world(
        &users_dir.join("Shared"),
        "bbbbbbbbbbb=",
        "Shared World",
        [1, 20, 80, 0, 0],
    )?;

    // World 3: For restore test (version 1.21.10.0)
    create_account_world(
        &users_dir.join("222222222222222222"),
        "ccccccccccc=",
        "Restore Test World",
        [1, 21, 10, 0, 0],
    )?;

    println!("Generated 3 test worlds in {}", users_dir.display());
    Ok(())
}

fn create_account_world(
    account_dir: &Path,
    folder_name: &str,
    level_name: &str,
    version: [u16; 5],
) -> Result<(), Box<dyn std::error::Error>> {
    let world_dir = account_dir
        .join("games")
        .join("com.mojang")
        .join("minecraftWorlds")
        .join(folder_name);
    create_world_files(&world_dir, folder_name, level_name, version)?;
    println!("Created account world: {} ({:?})", level_name, world_dir);
    Ok(())
}

fn create_shared_world(
    shared_dir: &Path,
    folder_name: &str,
    level_name: &str,
    version: [u16; 5],
) -> Result<(), Box<dyn std::error::Error>> {
    let world_dir = shared_dir
        .join("games")
        .join("com.mojang")
        .join("minecraftWorlds")
        .join(folder_name);
    create_world_files(&world_dir, folder_name, level_name, version)?;
    println!("Created shared world: {} ({:?})", level_name, world_dir);
    Ok(())
}

fn create_world_files(
    world_dir: &Path,
    _folder_name: &str,
    level_name: &str,
    version: [u16; 5],
) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(world_dir)?;

    // Build NBT structure: root Compound("Data") -> IntArray("lastOpenedWithVersion")
    let mut data_compound = BTreeMap::new();
    data_compound.insert(
        "lastOpenedWithVersion".to_string(),
        NbtValue::IntArray(version.iter().map(|&v| v as i32).collect()),
    );

    let mut root_compound = BTreeMap::new();
    root_compound.insert("Data".to_string(), NbtValue::Compound(data_compound));

    let root_nbt = NbtValue::Compound(root_compound);

    // Serialize NBT to binary
    let nbt_bytes = serialize_nbt(&root_nbt)?;

    // Build level.dat: header + NBT payload
    let header = LevelDatHeader {
        version: 123, // Bedrock version marker
        nbt_size: nbt_bytes.len() as i32,
    };

    let mut level_dat = Vec::new();
    level_dat.write_all(&header.version.to_le_bytes())?;
    level_dat.write_all(&header.nbt_size.to_le_bytes())?;
    level_dat.write_all(&nbt_bytes)?;

    // Write level.dat
    fs::write(world_dir.join("level.dat"), level_dat)?;

    // Write levelname.txt
    fs::write(world_dir.join("levelname.txt"), level_name)?;

    // Create a dummy world_icon.jpeg (1x1 pixel JPEG)
    let jpeg_bytes = vec![
        0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x01, 0x00,
        0x48, 0x00, 0x48, 0x00, 0x00, 0xFF, 0xDB, 0x00, 0x43, 0x00, 0x08, 0x06, 0x06, 0x07, 0x06,
        0x05, 0x08, 0x07, 0x07, 0x07, 0x09, 0x09, 0x08, 0x0A, 0x0C, 0x14, 0x0D, 0x0C, 0x0B, 0x0B,
        0x0C, 0x19, 0x12, 0x13, 0x0F, 0x14, 0x1D, 0x1A, 0x1F, 0x1E, 0x1D, 0x1A, 0x1C, 0x1C, 0x20,
        0x24, 0x2E, 0x27, 0x20, 0x22, 0x2C, 0x23, 0x1C, 0x1C, 0x28, 0x37, 0x29, 0x2C, 0x30, 0x31,
        0x34, 0x34, 0x34, 0x1F, 0x27, 0x39, 0x3D, 0x38, 0x32, 0x3C, 0x2E, 0x33, 0x34, 0x32, 0xFF,
        0xC0, 0x00, 0x11, 0x08, 0x00, 0x01, 0x00, 0x01, 0x03, 0x01, 0x22, 0x00, 0x02, 0x11, 0x01,
        0x03, 0x11, 0x01, 0xFF, 0xC4, 0x00, 0x1F, 0x00, 0x00, 0x01, 0x05, 0x01, 0x01, 0x01, 0x01,
        0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
        0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0xFF, 0xC4, 0x00, 0xB5, 0x10, 0x00, 0x02, 0x01, 0x03,
        0x03, 0x02, 0x04, 0x03, 0x05, 0x05, 0x04, 0x04, 0x00, 0x00, 0x01, 0x7D, 0x01, 0x02, 0x03,
        0x00, 0x04, 0x11, 0x05, 0x12, 0x21, 0x31, 0x41, 0x06, 0x13, 0x51, 0x61, 0x07, 0x22, 0x71,
        0x14, 0x32, 0x81, 0x91, 0xA1, 0x08, 0x23, 0x42, 0xB1, 0xC1, 0x15, 0x52, 0xD1, 0xF0, 0x24,
        0x33, 0x62, 0x72, 0x82, 0x09, 0x0A, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x25, 0x26, 0x27, 0x28,
        0x29, 0x2A, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48,
        0x49, 0x4A, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5A, 0x63, 0x64, 0x65, 0x66, 0x67,
        0x68, 0x69, 0x6A, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7A, 0x83, 0x84, 0x85, 0x86,
        0x87, 0x88, 0x89, 0x8A, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9A, 0xA2, 0xA3,
        0xA4, 0xA5, 0xA6, 0xA7, 0xA8, 0xA9, 0xAA, 0xB2, 0xB3, 0xB4, 0xB5, 0xB6, 0xB7, 0xB8, 0xB9,
        0xBA, 0xC2, 0xC3, 0xC4, 0xC5, 0xC6, 0xC7, 0xC8, 0xC9, 0xCA, 0xD2, 0xD3, 0xD4, 0xD5, 0xD6,
        0xD7, 0xD8, 0xD9, 0xDA, 0xE1, 0xE2, 0xE3, 0xE4, 0xE5, 0xE6, 0xE7, 0xE8, 0xE9, 0xEA, 0xF1,
        0xF2, 0xF3, 0xF4, 0xF5, 0xF6, 0xF7, 0xF8, 0xF9, 0xFA, 0xFF, 0xDA, 0x00, 0x0C, 0x03, 0x01,
        0x00, 0x02, 0x11, 0x03, 0x11, 0x00, 0x3F, 0x00, 0xF9, 0xFF, 0xD9,
    ];
    fs::write(world_dir.join("world_icon.jpeg"), jpeg_bytes)?;

    Ok(())
}

fn serialize_nbt(value: &NbtValue) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buf = Vec::new();
    write_nbt(&mut buf, "", value)?;
    Ok(buf)
}

fn write_nbt(
    buf: &mut Vec<u8>,
    name: &str,
    value: &NbtValue,
) -> Result<(), Box<dyn std::error::Error>> {
    // Write tag ID
    buf.push(value.tag_type().id());

    // Write name (for non-root compounds)
    if !name.is_empty() {
        let name_bytes = name.as_bytes();
        buf.write_all(&(name_bytes.len() as u16).to_le_bytes())?;
        buf.write_all(name_bytes)?;
    }

    // Write payload
    match value {
        NbtValue::Byte(v) => buf.write_all(&v.to_le_bytes())?,
        NbtValue::Short(v) => buf.write_all(&v.to_le_bytes())?,
        NbtValue::Int(v) => buf.write_all(&v.to_le_bytes())?,
        NbtValue::Long(v) => buf.write_all(&v.to_le_bytes())?,
        NbtValue::Float(v) => buf.write_all(&v.to_le_bytes())?,
        NbtValue::Double(v) => buf.write_all(&v.to_le_bytes())?,
        NbtValue::ByteArray(arr) => {
            buf.write_all(&(arr.len() as i32).to_le_bytes())?;
            buf.write_all(arr)?;
        }
        NbtValue::String(s) => {
            let bytes = s.as_bytes();
            buf.write_all(&(bytes.len() as u16).to_le_bytes())?;
            buf.write_all(bytes)?;
        }
        NbtValue::List(list) => {
            buf.push(list.element_type.id());
            buf.write_all(&(list.values.len() as i32).to_le_bytes())?;
            for item in &list.values {
                write_nbt(buf, "", item)?;
            }
        }
        NbtValue::Compound(map) => {
            for (key, val) in map {
                write_nbt(buf, key, val)?;
            }
            buf.push(NbtTagType::End.id());
        }
        NbtValue::IntArray(arr) => {
            buf.write_all(&(arr.len() as i32).to_le_bytes())?;
            for v in arr {
                buf.write_all(&v.to_le_bytes())?;
            }
        }
        NbtValue::LongArray(arr) => {
            buf.write_all(&(arr.len() as i32).to_le_bytes())?;
            for v in arr {
                buf.write_all(&v.to_le_bytes())?;
            }
        }
    }
    Ok(())
}
