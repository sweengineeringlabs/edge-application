//! [`SnapshotLoadRequest`] — request identifying an aggregate to load the latest snapshot for.
// @allow: dto_types_must_serialize — holds a borrowed `&'a Id` reference, not
// owned wire-format data; a derived Deserialize cannot produce a borrowed
// reference with an unbounded lifetime.

/// Request to load the latest snapshot for `id`.
pub struct SnapshotLoadRequest<'a, Id> {
    /// The aggregate id to look up.
    pub id: &'a Id,
}
