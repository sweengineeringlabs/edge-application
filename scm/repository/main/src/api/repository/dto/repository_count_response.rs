//! [`RepositoryCountResponse`] — wrapper for a repository entity count.

/// Result of [`Repository::count`](crate::api::repository::traits::Repository::count).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RepositoryCountResponse {
    /// The total number of entities in the repository.
    pub count: usize,
}
