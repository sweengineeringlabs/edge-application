//! [`EmptinessRequest`] — zero-sized marker for querying if the registry is empty.

/// Request for whether the registry has any registered entries.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EmptinessRequest;
