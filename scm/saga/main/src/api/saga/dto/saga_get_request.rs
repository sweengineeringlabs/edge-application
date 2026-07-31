//! [`SagaGetRequest`] — request identifying a saga instance by id.
// @allow: dto_types_must_serialize — holds a borrowed `&'a Id` reference, not
// owned wire-format data; a derived Deserialize cannot produce a borrowed
// reference with an unbounded lifetime.

/// Request to borrow the saga registered under `id`.
pub struct SagaGetRequest<'a, Id> {
    /// The identifier to look up.
    pub id: &'a Id,
}
