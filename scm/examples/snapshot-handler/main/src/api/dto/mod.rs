//! DTOs for the two `Handler`s wired to a shared `SnapshotStore<String, OrderSnap>`.

mod snapshot_lookup_request;
mod snapshot_lookup_response;
mod snapshot_persistence_request;
mod snapshot_persistence_response;

pub use snapshot_lookup_request::SnapshotLookupRequest;
pub use snapshot_lookup_response::SnapshotLookupResponse;
pub use snapshot_persistence_request::SnapshotPersistenceRequest;
pub use snapshot_persistence_response::SnapshotPersistenceResponse;
