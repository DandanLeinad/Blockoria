// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let state = match blockoria_tauri_lib::create_state() {
        Ok(state) => state,
        Err(err) => {
            // Spec 07, RF-08: never die silently — release builds hide the
            // console (windows_subsystem), so the user sees nothing unless
            // we surface the failure in a native dialog first.
            rfd::MessageDialog::new()
                .set_title("Blockoria")
                .set_level(rfd::MessageLevel::Error)
                .set_description(format!("Não foi possível iniciar o Blockoria.\n\n{err}"))
                .show();
            std::process::exit(1);
        }
    };
    blockoria_tauri_lib::run_with_state(state);
}
