//! `SnapshotLookupHandler` — holds the *same* injected `SnapshotStore` instance as
//! `SnapshotPersistenceHandler`, one store, multiple handlers, each reaching it through their
//! own constructor-injected field.

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_handler::{ExecutionRequest as HandlerExecutionRequest, Handler, HandlerError};
use edge_application_snapshot::{SnapshotLoadRequest, SnapshotStore};

use crate::api::{OrderSnap, SnapshotLookupRequest, SnapshotLookupResponse};

pub struct SnapshotLookupHandler {
    pub snapshot_store: Arc<dyn SnapshotStore<AggregateId = String, Snap = OrderSnap>>,
}

#[async_trait]
impl Handler for SnapshotLookupHandler {
    type Request = SnapshotLookupRequest;
    type Response = SnapshotLookupResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, SnapshotLookupRequest>,
    ) -> Result<SnapshotLookupResponse, HandlerError> {
        tracing::info!(
            "[1] SnapshotLookupHandler::execute — delegating to its own injected SnapshotStore"
        );
        let loaded = self
            .snapshot_store
            .load(SnapshotLoadRequest {
                id: &req.req.aggregate_id,
            })
            .await
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        let snapshot = loaded.snapshot.map(|s| (s.version, s.total));
        tracing::info!("[2] SnapshotLookupHandler::execute — snapshot store returned {snapshot:?}");
        Ok(SnapshotLookupResponse { snapshot })
    }
}
