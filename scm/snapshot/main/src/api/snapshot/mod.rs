//! `Snapshot` theme — point-in-time aggregate state for replay optimisation.

pub mod dto;
pub mod errors;
pub mod memory_snapshot_store;
pub mod noop_snapshot;
pub mod traits;

pub use dto::{
    SnapshotAggregateIdRequest, SnapshotAggregateIdResponse, SnapshotLoadRequest,
    SnapshotLoadResponse, SnapshotSaveRequest, SnapshotVersionRequest, SnapshotVersionResponse,
};
pub use errors::SnapshotError;
pub use memory_snapshot_store::MemorySnapshotStore;
pub use noop_snapshot::NoopSnapshot;
pub use traits::{Snapshot, SnapshotStore};
