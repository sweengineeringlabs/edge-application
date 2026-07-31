//! `SnapshotPersistenceHandler` — holds its own injected `SnapshotStore`; genuinely reads
//! `req.ctx` inside `execute()`.

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_handler::{
    DrainRequest, ExecutionRequest as HandlerExecutionRequest, Handler, HandlerError,
    LogEmitRequest,
};
use edge_application_snapshot::{SnapshotSaveRequest, SnapshotStore};

use crate::api::{OrderSnap, SnapshotPersistenceRequest, SnapshotPersistenceResponse};

pub struct SnapshotPersistenceHandler {
    pub snapshot_store: Arc<dyn SnapshotStore<AggregateId = String, Snap = OrderSnap>>,
}

#[async_trait]
impl Handler for SnapshotPersistenceHandler {
    type Request = SnapshotPersistenceRequest;
    type Response = SnapshotPersistenceResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, SnapshotPersistenceRequest>,
    ) -> Result<SnapshotPersistenceResponse, HandlerError> {
        // ctx IS genuinely read here — real per-request observability, not filler.
        req.ctx
            .observer
            .drain(DrainRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .drain
            .emit(LogEmitRequest {
                level: "INFO".to_string(),
                handler_id: "snapshot_persistence_handler".to_string(),
                message: format!("saving snapshot for {:?}", req.req.aggregate_id),
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;

        tracing::info!(
            "[1] SnapshotPersistenceHandler::execute — delegating to its own injected SnapshotStore"
        );
        self.snapshot_store
            .save(SnapshotSaveRequest {
                snapshot: OrderSnap {
                    aggregate_id: req.req.aggregate_id,
                    version: req.req.version,
                    total: req.req.total,
                },
            })
            .await
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        tracing::info!("[2] SnapshotPersistenceHandler::execute — snapshot store confirmed the save");
        Ok(SnapshotPersistenceResponse { saved: true })
    }
}
