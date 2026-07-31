//! # edge-domain-snapshot
//!
//! The `Snapshot` port contract — point-in-time aggregate state capture for event replay optimisation.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use api::MemorySnapshotStore;
pub use api::NoopSnapshot;
pub use api::SnapshotAggregateIdRequest;
pub use api::SnapshotAggregateIdResponse;
pub use api::SnapshotError;
pub use api::SnapshotLoadRequest;
pub use api::SnapshotLoadResponse;
pub use api::SnapshotSaveRequest;
pub use api::SnapshotVersionRequest;
pub use api::SnapshotVersionResponse;
pub use saf::Snapshot;
pub use saf::SnapshotStore;
pub use saf::SNAPSHOT_STORE_SVC;
pub use saf::SNAPSHOT_STORE_SVC_FACTORY;
pub use saf::SNAPSHOT_SVC;
pub use saf::SNAPSHOT_SVC_FACTORY;
