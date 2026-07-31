//! [`GaugeLookupRequest`] — request to resolve a named gauge instrument.

/// Request for the [`Gauge`](crate::api::context::observe::Gauge) instrument named `name`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GaugeLookupRequest {
    /// The instrument name.
    pub name: String,
}
