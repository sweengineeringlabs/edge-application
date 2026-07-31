//! `AlwaysValid` — a null-object [`Validator`](crate::Validator) that always passes.

/// Reference [`Validator`](crate::Validator) implementation that accepts every
/// configuration.
///
/// Use as a null-object default where a `Validator` is required but no checks
/// are needed (e.g. optional config sections that are always structurally valid).
///
/// The concrete trait impl lives in `core::validator::always_valid`.
pub struct AlwaysValid;
