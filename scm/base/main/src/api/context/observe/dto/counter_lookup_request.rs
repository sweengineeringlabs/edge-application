//! [`CounterLookupRequest`] — request to resolve a named counter instrument.

/// Request for the [`Counter`](crate::api::context::observe::Counter) instrument named `name`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CounterLookupRequest {
    /// The instrument name.
    pub name: String,
}
