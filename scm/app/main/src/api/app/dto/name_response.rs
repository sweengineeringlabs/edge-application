//! [`NameResponse`] — wrapper for a stable identifier.

/// A stable identifier for an `AppRuntime`, `AppServiceProvider`, or `Application`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct NameResponse {
    /// The identifier.
    pub name: &'static str,
}
