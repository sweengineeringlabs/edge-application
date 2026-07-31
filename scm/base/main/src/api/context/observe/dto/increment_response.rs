//! [`IncrementResponse`] — wrapper for a successful counter increment.

/// Result of [`Counter::increment`](crate::api::context::observe::Counter::increment).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct IncrementResponse;
