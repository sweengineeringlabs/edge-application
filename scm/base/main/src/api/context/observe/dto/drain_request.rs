//! [`DrainRequest`] — zero-sized marker for querying the active `LogDrain`.

/// Request for the active [`LogDrain`](crate::api::context::observe::LogDrain).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DrainRequest;
