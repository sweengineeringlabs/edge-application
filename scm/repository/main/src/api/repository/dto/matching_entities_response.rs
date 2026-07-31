//! [`MatchingEntitiesResponse`] — wrapper for a specification-filtered entity list.

/// Result of [`QueryableRepository::find_by`](crate::api::repository::traits::QueryableRepository::find_by).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchingEntitiesResponse<T> {
    /// Entities that matched the specification.
    pub items: Vec<T>,
}
