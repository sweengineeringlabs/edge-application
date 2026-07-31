//! [`ListIdsRequest`] — zero-sized marker for listing registered ids.

/// Request to list all registered ids.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ListIdsRequest;
