//! [`IdRequest`] — zero-sized marker for querying an entity's identifier.

/// Request for an entity's stable identifier.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct IdRequest;
