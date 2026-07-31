//! [`SagaGetResponse`] — wrapper borrowing the saga instance found by id.
// @allow: dto_types_must_serialize — holds a borrowed `&'a S` reference, not
// owned wire-format data; a derived Deserialize cannot produce a borrowed
// reference with an unbounded lifetime.

/// Result of [`SagaStore::get`](crate::api::saga::traits::SagaStore::get).
#[derive(Debug)]
pub struct SagaGetResponse<'a, S> {
    /// The saga registered under the requested id.
    pub saga: &'a S,
}
