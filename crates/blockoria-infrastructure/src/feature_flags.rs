// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! Code-based feature flags (spec 08).
//!
//! Flag defaults live in code ([`Flag::default_enabled`]), never in
//! user-editable config: launch flags hide incomplete work, so the end user
//! must not be able to flip them. In debug builds only, the
//! `BLOCKORIA_FLAGS` environment variable (comma-separated flag names)
//! overrides the default — the env read is not compiled in release.
//!
//! [`Flag::default_enabled`]: blockoria_application::ports::Flag::default_enabled

use blockoria_application::ports::{FeatureFlags, Flag};

/// Feature flags resolved from code defaults (spec 08).
///
/// Stateless: every check reads the flag's built-in default plus, in debug
/// builds only, the `BLOCKORIA_FLAGS` environment variable.
pub struct CodeFeatureFlags;

impl FeatureFlags for CodeFeatureFlags {
    fn is_enabled(&self, flag: Flag) -> bool {
        #[cfg(debug_assertions)]
        let env_list = std::env::var("BLOCKORIA_FLAGS").ok();
        #[cfg(not(debug_assertions))]
        let env_list: Option<String> = None;

        resolve(flag.name(), env_list.as_deref(), flag.default_enabled())
    }
}

/// Pure resolution logic — the unit-test target (spec 08, RF-02).
///
/// `env_list` is passed in explicitly (never read here) so tests cover every
/// branch without touching process state (`set_var` is unsafe in edition
/// 2024 and racy across test threads).
fn resolve(name: &str, env_list: Option<&str>, default: bool) -> bool {
    if let Some(list) = env_list {
        for token in list.split(',').map(str::trim).filter(|t| !t.is_empty()) {
            if token == name {
                return true;
            }
            if !Flag::ALL.iter().any(|f| f.name() == token) {
                // Unknown token: warn instead of failing silently — a typo in
                // the env var must not cost a silent debugging session.
                eprintln!("BLOCKORIA_FLAGS: '{token}' is not a known feature flag");
            }
        }
    }
    default
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_no_env_override_when_resolved_then_uses_default() {
        // Given
        let name = "minha_flag";
        let default = false;

        // When
        let resolved = resolve(name, None, default);

        // Then
        assert_eq!(resolved, default);
    }

    #[test]
    fn given_default_true_and_no_env_when_resolved_then_true() {
        // Given
        let default = true;

        // When
        let resolved = resolve("minha_flag", None, default);

        // Then
        assert!(resolved);
    }

    #[test]
    fn given_env_listing_the_flag_when_resolved_then_enabled() {
        // Given
        let list = "minha_flag";

        // When
        let resolved = resolve("minha_flag", Some(list), false);

        // Then
        assert!(resolved);
    }

    #[test]
    fn given_env_with_multiple_spaced_tokens_when_resolved_then_listed_flag_enabled() {
        // Given
        let list = "outra_flag, minha_flag ,terceira_flag";

        // When
        let resolved = resolve("minha_flag", Some(list), false);

        // Then
        assert!(resolved);
    }

    #[test]
    fn given_env_without_the_flag_when_resolved_then_uses_default() {
        // Given
        let list = "outra_flag,flag_inexistente";

        // When
        let resolved = resolve("minha_flag", Some(list), false);

        // Then
        assert!(!resolved);
    }

    #[test]
    fn given_blank_env_when_resolved_then_uses_default() {
        // Given
        let list = " , ";

        // When
        let resolved = resolve("minha_flag", Some(list), false);

        // Then
        assert!(!resolved);
    }
}
