//! `OrderLookupHandler` — holds the *same* injected `Repository` instance as
//! `OrderPersistenceHandler`, one repository, multiple handlers, each reaching it through their
//! own constructor-injected field.

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_handler::{ExecutionRequest as HandlerExecutionRequest, Handler, HandlerError};
use edge_application_repository::{Repository, RepositoryIdRequest};

use crate::api::{Order, OrderLookupRequest, OrderLookupResponse};

pub struct OrderLookupHandler {
    pub repository: Arc<dyn Repository<Entity = Order, Id = String>>,
}

#[async_trait]
impl Handler for OrderLookupHandler {
    type Request = OrderLookupRequest;
    type Response = OrderLookupResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, OrderLookupRequest>,
    ) -> Result<OrderLookupResponse, HandlerError> {
        tracing::info!("[1] OrderLookupHandler::execute — delegating to its own injected Repository");
        let found = self
            .repository
            .find(RepositoryIdRequest {
                id: &req.req.order_id,
            })
            .await
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        tracing::info!(
            "[2] OrderLookupHandler::execute — repository returned {:?}",
            found.entity
        );
        Ok(OrderLookupResponse {
            order: found.entity,
        })
    }
}
