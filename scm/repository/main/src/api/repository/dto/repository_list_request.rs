//! [`RepositoryListRequest`] — zero-sized marker for listing/counting all entities.

/// Request to list or count all entities in a repository.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct RepositoryListRequest;
