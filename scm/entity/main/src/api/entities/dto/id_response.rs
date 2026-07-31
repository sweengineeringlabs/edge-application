//! [`IdResponse`] — wrapper for an entity's identifier.
// @allow: dto_types_must_serialize — holds a borrowed `&'a I` reference into the
// entity, not owned wire-format data; a derived Deserialize cannot produce a
// borrowed reference with an unbounded lifetime.

/// Result of [`Entity::id`](crate::api::Entity::id).
pub struct IdResponse<'a, I> {
    /// The entity's stable identifier.
    pub id: &'a I,
}
