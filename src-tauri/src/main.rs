// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let state = blockoria_tauri_lib::create_state().expect("failed to create app state");
    blockoria_tauri_lib::run_with_state(state);
}
