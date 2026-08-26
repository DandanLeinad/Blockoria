// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

use blockoria_domain::DomainError;

pub type Result<T> = std::result::Result<T, DomainError>;
