//! [`NameRequest`] — zero-sized marker for querying a stable identifier.

/// Request for a stable identifier.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct NameRequest;
