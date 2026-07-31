//! API layer — DTOs for the two `Handler`s wired to a shared `SnapshotStore<String, OrderSnap>`.

mod dto;
mod order_snap;

pub use dto::{
    SnapshotLookupRequest, SnapshotLookupResponse, SnapshotPersistenceRequest,
    SnapshotPersistenceResponse,
};
pub use order_snap::OrderSnap;
