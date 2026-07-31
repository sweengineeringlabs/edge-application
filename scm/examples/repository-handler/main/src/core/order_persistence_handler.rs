//! `OrderPersistenceHandler` — holds its own injected `Repository`; genuinely reads `req.ctx`
//! inside `execute()`.

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_handler::{
    DrainRequest, ExecutionRequest as HandlerExecutionRequest, Handler, HandlerError,
    LogEmitRequest,
};
use edge_application_repository::{Repository, RepositorySaveRequest};

use crate::api::{Order, OrderPersistenceRequest, OrderPersistenceResponse};

pub struct OrderPersistenceHandler {
    pub repository: Arc<dyn Repository<Entity = Order, Id = String>>,
}

#[async_trait]
impl Handler for OrderPersistenceHandler {
    type Request = OrderPersistenceRequest;
    type Response = OrderPersistenceResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, OrderPersistenceRequest>,
    ) -> Result<OrderPersistenceResponse, HandlerError> {
        // ctx IS genuinely read here — real per-request observability, not filler.
        req.ctx
            .observer
            .drain(DrainRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .drain
            .emit(LogEmitRequest {
                level: "INFO".to_string(),
                handler_id: "order_persistence_handler".to_string(),
                message: format!("saving order {:?}", req.req.order_id),
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;

        tracing::info!("[1] OrderPersistenceHandler::execute — delegating to its own injected Repository");
        self.repository
            .save(RepositorySaveRequest {
                id: req.req.order_id,
                entity: Order {
                    item: req.req.item,
                    quantity: req.req.quantity,
                },
            })
            .await
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        tracing::info!("[2] OrderPersistenceHandler::execute — repository confirmed the save");
        Ok(OrderPersistenceResponse { saved: true })
    }
}
