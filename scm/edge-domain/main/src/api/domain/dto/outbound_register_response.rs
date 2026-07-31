//! [`OutboundRegisterResponse`] — wrapper for a successful handle registration.

/// Result of [`OutboundRegistry::register`](crate::api::domain::traits::OutboundRegistry::register).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct OutboundRegisterResponse;
