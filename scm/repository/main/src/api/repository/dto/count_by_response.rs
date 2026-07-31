//! [`CountByResponse`] — wrapper for a specification-filtered entity count.

/// Result of [`QueryableRepository::count_by`](crate::api::repository::traits::QueryableRepository::count_by).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CountByResponse {
    /// The number of entities that matched the specification.
    pub count: usize,
}
