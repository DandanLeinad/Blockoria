// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! Test binary to parse a level.dat file and extract WorldVersion.
//! Run with: cargo run --example test_level_dat -- <path/to/level.dat>

use blockoria_infrastructure::nbt::{LevelDatParser, extract_world_version};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <path/to/level.dat>", args[0]);
        std::process::exit(1);
    }

    let path = PathBuf::from(&args[1]);

    println!("Parsing level.dat: {}", path.display());

    // Read and parse
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            std::process::exit(1);
        }
    };

    let mut reader = BufReader::new(file);
    let parser = match LevelDatParser::new(&mut reader) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to parse header: {}", e);
            std::process::exit(1);
        }
    };

    println!(
        "Header: version={}, nbt_size={}",
        parser.header().version,
        parser.header().nbt_size
    );

    let nbt = match parser.parse() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Failed to parse NBT: {}", e);
            std::process::exit(1);
        }
    };

    // Extract version
    let version = extract_world_version(&nbt);

    match version {
        Some(v) => {
            println!("WorldVersion: {:?}", v.as_array());
            println!(
                "WorldVersion (formatted): {}.{}.{}.{}.{}",
                v.as_array()[0],
                v.as_array()[1],
                v.as_array()[2],
                v.as_array()[3],
                v.as_array()[4]
            );
        }
        None => {
            println!("WorldVersion: NOT FOUND (lastOpenedWithVersion missing or invalid)");
        }
    }

    // Optionally print full NBT structure (simple JSON)
    {
        let json = blockoria_infrastructure::nbt::to_json_simple(&nbt);
        println!("\nFull NBT (simple):");
        println!("{}", serde_json::to_string_pretty(&json).unwrap());
    }
}
