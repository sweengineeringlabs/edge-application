//! [`ValidationRequest`] — zero-sized marker for requesting entity invariant validation.

/// Request to validate an entity's invariants.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ValidationRequest;
