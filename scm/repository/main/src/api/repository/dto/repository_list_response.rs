//! [`RepositoryListResponse`] — wrapper for a full entity listing.

/// Result of [`Repository::list`](crate::api::repository::traits::Repository::list).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryListResponse<T> {
    /// All entities in the repository.
    pub items: Vec<T>,
}
