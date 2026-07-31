//! [`SagaHandleRequest`] — request to apply an event to a saga.
// @allow: dto_types_must_serialize — holds a borrowed `&'a E` reference, not
// owned wire-format data; a derived Deserialize cannot produce a borrowed
// reference with an unbounded lifetime.

/// Request to apply `event` to a [`Saga`](crate::api::saga::traits::Saga).
pub struct SagaHandleRequest<'a, E> {
    /// The event to apply.
    pub event: &'a E,
}
