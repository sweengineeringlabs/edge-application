//! [`ElapsedSinceEpochRequest`] — zero-sized marker for querying elapsed time since the Unix epoch.

/// Request for the elapsed duration since the Unix epoch.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ElapsedSinceEpochRequest;
