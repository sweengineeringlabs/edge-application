//! `NoopSnapshot` — a zero-cost [`Snapshot`](crate::Snapshot) for structural compliance.

/// A no-op [`Snapshot`](crate::Snapshot) implementation used for testing and structural compliance.
#[derive(Debug, Default, Clone)]
pub struct NoopSnapshot {
    pub(crate) id: String,
    pub(crate) version: u64,
}
