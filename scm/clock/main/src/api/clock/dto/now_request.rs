//! [`NowRequest`] — zero-sized marker for querying the current instant.

/// Request for the current instant.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct NowRequest;
