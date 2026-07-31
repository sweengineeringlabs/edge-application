//! [`ValidationRequest`] — zero-sized marker for re-checking a value object's invariants.

/// Request to re-validate a [`ValueObject`](crate::api::ValueObject)'s invariants.
///
/// Useful after deserialization or other construction paths that bypass the
/// type's normal constructor.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ValidationRequest;
