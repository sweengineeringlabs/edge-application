//! [`DeregisterRequest`] — request to remove the entry registered under an id.

/// Request to remove the entry registered under `id`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DeregisterRequest {
    /// The id to remove.
    pub id: String,
}
