//! [`RepositoryFindResponse`] — wrapper for a repository lookup result.

/// Result of [`Repository::find`](crate::api::repository::traits::Repository::find).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryFindResponse<T> {
    /// The entity, if it exists.
    pub entity: Option<T>,
}
