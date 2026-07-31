//! [`ValidationResponse`] — wrapper for a successful backend reachability check.

/// Result of [`ObserveBootstrap::validate`](crate::api::ObserveBootstrap::validate).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ValidationResponse;
