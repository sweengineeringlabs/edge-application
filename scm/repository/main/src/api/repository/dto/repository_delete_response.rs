//! [`RepositoryDeleteResponse`] — wrapper for a delete outcome.

/// Result of [`Repository::delete`](crate::api::repository::traits::Repository::delete).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RepositoryDeleteResponse {
    /// `true` if an entity was removed.
    pub removed: bool,
}
