//! [`OutboundNamesRequest`] — zero-sized marker for listing registered names.

/// Request to list all registered handle names.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct OutboundNamesRequest;
