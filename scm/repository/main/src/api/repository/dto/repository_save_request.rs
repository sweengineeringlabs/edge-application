//! [`RepositorySaveRequest`] — request to persist an entity under an id.

/// Request to persist `entity` under `id`, replacing any existing entry.
#[derive(Debug, Clone)]
pub struct RepositorySaveRequest<Id, T> {
    /// The identifier to store the entity under.
    pub id: Id,
    /// The entity to persist.
    pub entity: T,
}
