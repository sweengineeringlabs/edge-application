//! [`SnapshotAggregateIdResponse`] — wrapper borrowing a snapshot's aggregate id.
// @allow: dto_types_must_serialize — holds a borrowed `&'a Id` reference, not
// owned wire-format data; a derived Deserialize cannot produce a borrowed
// reference with an unbounded lifetime.

/// Result of [`Snapshot::aggregate_id`](crate::api::Snapshot::aggregate_id).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SnapshotAggregateIdResponse<'a, Id> {
    /// The aggregate id this snapshot belongs to.
    pub aggregate_id: &'a Id,
}
