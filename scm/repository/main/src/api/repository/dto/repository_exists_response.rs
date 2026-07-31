//! [`RepositoryExistsResponse`] — wrapper for an existence check.

/// Result of [`Repository::exists`](crate::api::repository::traits::Repository::exists).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RepositoryExistsResponse {
    /// `true` if an entity with the requested id exists.
    pub exists: bool,
}
