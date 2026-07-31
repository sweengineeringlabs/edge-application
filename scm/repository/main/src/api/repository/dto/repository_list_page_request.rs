//! [`RepositoryListPageRequest`] — request for a paginated slice of entities.

/// Request for a paginated slice of entities, starting at `offset` with at most `limit` items.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RepositoryListPageRequest {
    /// The zero-based offset to start the page at.
    pub offset: usize,
    /// The maximum number of items to return.
    pub limit: usize,
}
