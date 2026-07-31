//! [`RegistryLookupRequest`] — request to resolve the entry registered under an id.

/// Request to resolve the entry registered under `id`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RegistryLookupRequest {
    /// The id to resolve.
    pub id: String,
}
