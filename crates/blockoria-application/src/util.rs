// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! Shared utilities for the application layer.
//!
//! This module contains shared utility functions used by multiple use cases.
//! Currently provides filesystem operations for recursive directory copying.

use std::fs;
use std::path::Path;

/// Recursively copies a directory to a destination, overwriting existing files.
///
/// Used by `create_backup` and `restore_backup` use cases.
pub fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), std::io::Error> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}
