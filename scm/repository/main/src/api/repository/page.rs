//! `Page` — paginated result wrapper for repository list operations.

/// A paginated slice of results from a repository list operation.
#[derive(Debug, Clone)]
pub struct Page<T> {
    /// The items in this page.
    pub items: Vec<T>,
    /// Total number of matching entities across all pages.
    pub total: usize,
    /// The zero-based offset this page starts at.
    pub offset: usize,
    /// The maximum number of items per page that was requested.
    pub limit: usize,
}
