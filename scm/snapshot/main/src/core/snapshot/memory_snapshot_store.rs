//! `SnapshotStore` impl for [`MemorySnapshotStore`].

use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

use futures::future::BoxFuture;

use crate::api::MemorySnapshotStore;
use crate::api::SnapshotError;
use crate::api::{Snapshot, SnapshotStore};
use crate::api::{
    SnapshotAggregateIdRequest, SnapshotLoadRequest, SnapshotLoadResponse, SnapshotSaveRequest,
    SnapshotVersionRequest,
};

impl<S: Snapshot> MemorySnapshotStore<S>
where
    S::AggregateId: Eq + Hash,
{
    /// Create a new empty in-memory snapshot store.
    pub fn new() -> Self {
        Self {
            snapshots: std::sync::RwLock::new(HashMap::new()),
        }
    }
}

impl<S: Snapshot> Default for MemorySnapshotStore<S>
where
    S::AggregateId: Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<S> SnapshotStore for MemorySnapshotStore<S>
where
    S: Snapshot + Clone,
    S::AggregateId: Display,
{
    type AggregateId = S::AggregateId;
    type Snap = S;

    fn save(
        &self,
        req: SnapshotSaveRequest<Self::Snap>,
    ) -> BoxFuture<'_, Result<(), SnapshotError>> {
        let snapshot = req.snapshot;
        let version = match snapshot.version(SnapshotVersionRequest) {
            Ok(resp) => resp.version,
            Err(e) => return Box::pin(async move { Err(e) }),
        };
        if version == 0 {
            let aggregate_id = match snapshot.aggregate_id(SnapshotAggregateIdRequest) {
                Ok(resp) => resp.aggregate_id.to_string(),
                Err(e) => return Box::pin(async move { Err(e) }),
            };
            return Box::pin(async move {
                Err(SnapshotError::InvalidVersion {
                    aggregate_id,
                    version,
                })
            });
        }
        let key = match snapshot.aggregate_id(SnapshotAggregateIdRequest) {
            Ok(resp) => resp.aggregate_id.clone(),
            Err(e) => return Box::pin(async move { Err(e) }),
        };
        self.snapshots
            .write()
            .unwrap_or_else(|e| e.into_inner())
            .insert(key, snapshot);
        Box::pin(async move { Ok(()) })
    }

    fn load<'a>(
        &'a self,
        req: SnapshotLoadRequest<'a, Self::AggregateId>,
    ) -> BoxFuture<'a, Result<SnapshotLoadResponse<Self::Snap>, SnapshotError>> {
        let snapshot = self
            .snapshots
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .get(req.id)
            .cloned();
        Box::pin(async move { Ok(SnapshotLoadResponse { snapshot }) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{SnapshotAggregateIdResponse, SnapshotVersionResponse};

    #[derive(Clone)]
    struct MemorySnapshotStoreOrderFixture {
        aggregate_id: String,
        version: u64,
    }

    impl Snapshot for MemorySnapshotStoreOrderFixture {
        type AggregateId = String;
        fn aggregate_id(
            &self,
            _req: SnapshotAggregateIdRequest,
        ) -> Result<SnapshotAggregateIdResponse<'_, String>, SnapshotError> {
            Ok(SnapshotAggregateIdResponse {
                aggregate_id: &self.aggregate_id,
            })
        }
        fn version(
            &self,
            _req: SnapshotVersionRequest,
        ) -> Result<SnapshotVersionResponse, SnapshotError> {
            Ok(SnapshotVersionResponse {
                version: self.version,
            })
        }
    }

    fn fixture(id: &str, v: u64) -> MemorySnapshotStoreOrderFixture {
        MemorySnapshotStoreOrderFixture {
            aggregate_id: id.to_string(),
            version: v,
        }
    }

    #[test]
    fn test_save_new_snapshot_inserts_entry_happy() {
        let store = MemorySnapshotStore::new();
        futures::executor::block_on(store.save(SnapshotSaveRequest {
            snapshot: fixture("agg-1", 3),
        }))
        .unwrap();
        assert!(store
            .snapshots
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .contains_key("agg-1"));
    }

    #[test]
    fn test_save_version_zero_returns_invalid_version_error() {
        let store = MemorySnapshotStore::<MemorySnapshotStoreOrderFixture>::new();
        let err = futures::executor::block_on(store.save(SnapshotSaveRequest {
            snapshot: fixture("agg-1", 0),
        }))
        .unwrap_err();
        assert!(matches!(err, SnapshotError::InvalidVersion { .. }));
    }

    #[test]
    fn test_load_absent_aggregate_returns_none_edge() {
        let store = MemorySnapshotStore::<MemorySnapshotStoreOrderFixture>::new();
        let id = "absent".to_string();
        let result = futures::executor::block_on(store.load(SnapshotLoadRequest { id: &id }))
            .unwrap()
            .snapshot;
        assert!(result.is_none());
    }
}
