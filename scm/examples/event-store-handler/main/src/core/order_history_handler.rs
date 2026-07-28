//! `OrderHistoryHandler` — holds the *same* injected `EventStore` instance as
//! `OrderCreationHandler`, one event store, multiple handlers, each reaching it through their
//! own constructor-injected field.

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_event::{EventStore, EventStoreLoadRequest};
use edge_application_handler::{ExecutionRequest as HandlerExecutionRequest, Handler, HandlerError};

use crate::api::{OrderCreated, OrderHistoryRequest, OrderHistoryResponse};

pub struct OrderHistoryHandler {
    pub event_store: Arc<dyn EventStore<Event = OrderCreated>>,
}

#[async_trait]
impl Handler for OrderHistoryHandler {
    type Request = OrderHistoryRequest;
    type Response = OrderHistoryResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, OrderHistoryRequest>,
    ) -> Result<OrderHistoryResponse, HandlerError> {
        tracing::info!(
            "[1] OrderHistoryHandler::execute — delegating to its own injected EventStore"
        );
        let loaded = self
            .event_store
            .load(EventStoreLoadRequest {
                aggregate_id: &req.req.order_id,
            })
            .await
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        let items: Vec<String> = loaded
            .events
            .iter()
            .map(|envelope| format!("seq {}: {}", envelope.sequence, envelope.event.item))
            .collect();
        tracing::info!("[2] OrderHistoryHandler::execute — event store returned {items:?}");
        Ok(OrderHistoryResponse { items })
    }
}
