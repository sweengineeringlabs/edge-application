//! [`OutboundIsEmptyRequest`] — zero-sized marker for querying if the registry is empty.

/// Request to check whether the registry holds no handles.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct OutboundIsEmptyRequest;
