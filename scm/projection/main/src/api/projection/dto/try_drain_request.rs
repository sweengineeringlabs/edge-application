//! [`TryDrainRequest`] — request to fold a slice of events into a projection.
// @allow: dto_types_must_serialize — holds a borrowed `&'a [E]` slice, not owned
// wire-format data; a derived Deserialize cannot produce a borrowed reference
// with an unbounded lifetime.

/// Request to apply `events` to a [`Projection`](crate::api::projection::traits::Projection),
/// in order, via repeated [`apply`](crate::api::projection::traits::Projection::apply) calls.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TryDrainRequest<'a, E> {
    /// The events to fold into the read model, in order.
    pub events: &'a [E],
}
