//! [`RepositoryIdRequest`] — request identifying an entity by id.

/// Request to look up, check existence of, or delete an entity by `id`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RepositoryIdRequest<'a, Id> {
    /// The identifier to look up.
    pub id: &'a Id,
}
