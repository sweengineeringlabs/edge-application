//! [`MatchingEntityResponse`] — wrapper for the first specification-matching entity.

/// Result of [`QueryableRepository::find_one_by`](crate::api::repository::traits::QueryableRepository::find_one_by).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchingEntityResponse<T> {
    /// The first entity that matched the specification, if any.
    pub entity: Option<T>,
}
