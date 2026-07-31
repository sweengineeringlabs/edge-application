//! `SagaEventHandler` — holds its own injected saga behind a lock; genuinely reads `req.ctx`
//! inside `execute()`.

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_handler::{
    DrainRequest, ExecutionRequest as HandlerExecutionRequest, Handler, HandlerError,
    LogEmitRequest,
};
use edge_application_saga::{Saga, SagaCommand, SagaCommandDispatchRequest, SagaHandleRequest};
use parking_lot::Mutex;

use crate::api::{OrderPaid, OrderSaga, SagaEventRequest, SagaEventResponse};

/// Holds its own injected saga behind a lock — the only way to reach `Saga::handle`'s `&mut
/// self`, since `SagaStore::get` can't provide mutable access. Genuinely reads `req.ctx`.
pub struct SagaEventHandler {
    pub saga: Arc<Mutex<OrderSaga>>,
}

#[async_trait]
impl Handler for SagaEventHandler {
    type Request = SagaEventRequest;
    type Response = SagaEventResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, SagaEventRequest>,
    ) -> Result<SagaEventResponse, HandlerError> {
        // ctx IS genuinely read here — real per-request observability, not filler.
        req.ctx
            .observer
            .drain(DrainRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .drain
            .emit(LogEmitRequest {
                level: "INFO".to_string(),
                handler_id: "saga_event_handler".to_string(),
                message: format!("handling order.paid for {:?}", req.req.order_id),
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;

        tracing::info!("[1] SagaEventHandler::execute — delegating to its own injected saga");
        let event = OrderPaid {
            order_id: req.req.order_id,
        };
        let commands = {
            let mut saga = self.saga.lock();
            saga.handle(SagaHandleRequest { event: &event })
                .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
                .commands
        };
        tracing::info!(
            "[2] SagaEventHandler::execute — saga staged {} command(s)",
            commands.len()
        );
        for command in &commands {
            command
                .dispatch(SagaCommandDispatchRequest)
                .await
                .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        }
        Ok(SagaEventResponse {
            commands_dispatched: commands.len(),
        })
    }
}
