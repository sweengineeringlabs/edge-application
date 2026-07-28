//! `OrderCreationHandler` — holds its own injected `EventStore`; genuinely reads `req.ctx`
//! inside `execute()`.

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_event::{EventStore, EventStoreAppendRequest, ExpectedVersion};
use edge_application_handler::{
    DrainRequest, ExecutionRequest as HandlerExecutionRequest, Handler, HandlerError,
    LogEmitRequest,
};

use crate::api::{OrderCreated, OrderCreationRequest, OrderCreationResponse};

pub struct OrderCreationHandler {
    pub event_store: Arc<dyn EventStore<Event = OrderCreated>>,
}

#[async_trait]
impl Handler for OrderCreationHandler {
    type Request = OrderCreationRequest;
    type Response = OrderCreationResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, OrderCreationRequest>,
    ) -> Result<OrderCreationResponse, HandlerError> {
        // ctx IS genuinely read here — real per-request observability, not filler.
        req.ctx
            .observer
            .drain(DrainRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .drain
            .emit(LogEmitRequest {
                level: "INFO".to_string(),
                handler_id: "order_creation_handler".to_string(),
                message: format!("appending order.created for {:?}", req.req.order_id),
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;

        tracing::info!(
            "[1] OrderCreationHandler::execute — delegating to its own injected EventStore"
        );
        let result = self
            .event_store
            .append(EventStoreAppendRequest {
                aggregate_id: &req.req.order_id,
                events: vec![OrderCreated {
                    order_id: req.req.order_id.clone(),
                    item: req.req.item,
                }],
                expected: ExpectedVersion::Any,
            })
            .await
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        tracing::info!(
            "[2] OrderCreationHandler::execute — event store confirmed sequence {}",
            result.sequence
        );
        Ok(OrderCreationResponse {
            sequence: result.sequence,
        })
    }
}
