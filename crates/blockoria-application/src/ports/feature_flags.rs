// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! Feature flag port (spec 08).
//!
//! Launch flags hide incomplete work that is already merged to `main`
//! (trunk-based development). The port lives in `application`; the concrete
//! resolution lives in `infrastructure` ([`CodeFeatureFlags`] is defined
//! there). The `domain` layer never knows flags exist.
//!
//! [`CodeFeatureFlags`]: https://docs.rs/blockoria_infrastructure/latest/blockoria_infrastructure/struct.CodeFeatureFlags.html

/// A known feature flag.
///
/// The enum starts **empty** (spec 08): every flag is introduced by the spec
/// of the feature it gates — spec 09 adds the first one, `OrphanRestore`.
///
/// # Lifecycle rules (spec 08, RF-05)
///
/// - Every new variant MUST be mapped in [`Flag::name`] and
///   [`Flag::default_enabled`] — both matches are exhaustive **without a
///   wildcard arm**, so the compiler refuses an unmapped flag.
/// - Every new variant MUST be listed in [`Flag::ALL`] (frontend visibility).
/// - Graduating a flag = deleting the variant and following the compiler
///   errors to remove every use site and the disabled branch.
///
/// shortcut: `ALL` is a manual array (no strum/EnumIter — a dependency for
/// two lines); forgetting to list a variant means the frontend never sees
/// the flag, which is caught immediately during development.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flag {}

impl Flag {
    /// Every known flag — feeds `cmd_feature_flags` and the env-var warning.
    pub const ALL: &'static [Flag] = &[];

    /// Stable snake_case identifier (env-var token, IPC map key).
    ///
    /// Exaustive match **without a wildcard arm** by design: adding a variant
    /// without mapping it is a compile error (spec 08, RF-05).
    pub fn name(&self) -> &'static str {
        match *self {}
    }

    /// Built-in default. In release builds this value is final.
    ///
    /// Exaustive match **without a wildcard arm** by design: adding a variant
    /// forces an explicit default decision (spec 08, RF-05).
    pub fn default_enabled(&self) -> bool {
        match *self {}
    }
}

/// Port for reading feature flags.
///
/// Infallible by contract: a flag without any override resolves to its
/// [`Flag::default_enabled`]. There is intentionally no `set` method — launch
/// flags are not user settings (spec 08).
pub trait FeatureFlags: Send + Sync {
    /// Returns whether `flag` is enabled in this environment.
    fn is_enabled(&self, flag: Flag) -> bool;
}

#[cfg(test)]
mod tests {
    // The enum is empty in spec 08 (zero flags registered); there is nothing
    // callable to unit-test yet. First real flag (OrphanRestore) and its
    // end-to-end scenarios arrive with spec 09. Resolution logic is tested
    // in blockoria-infrastructure (pure `resolve` function).
}
