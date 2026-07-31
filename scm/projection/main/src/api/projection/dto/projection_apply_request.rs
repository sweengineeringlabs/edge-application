//! [`ProjectionApplyRequest`] — request to fold one event into a projection.
// @allow: dto_types_must_serialize — holds a borrowed `&'a E` reference to the
// event, not owned wire-format data; a derived Deserialize cannot produce a
// borrowed reference with an unbounded lifetime.

/// Request to apply `event` to a [`Projection`](crate::api::projection::traits::Projection).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProjectionApplyRequest<'a, E> {
    /// The event to fold into the read model.
    pub event: &'a E,
}
