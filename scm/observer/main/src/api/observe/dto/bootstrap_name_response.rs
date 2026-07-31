//! [`BootstrapNameResponse`] — wrapper for a bootstrap implementation's identifier.

/// Result of [`ObserveBootstrap::bootstrap_name`](crate::api::ObserveBootstrap::bootstrap_name)
/// or [`NoopObserve::noop_name`](crate::api::NoopObserve::noop_name).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct BootstrapNameResponse {
    /// Stable identifier for this implementation.
    pub name: &'static str,
}
