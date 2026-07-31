//! [`ListIdsResponse`] — wrapper for the registered-id listing.

/// Result of [`Registry::list_ids`](crate::api::Registry::list_ids).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ListIdsResponse {
    /// All registered ids.
    pub ids: Vec<String>,
}
