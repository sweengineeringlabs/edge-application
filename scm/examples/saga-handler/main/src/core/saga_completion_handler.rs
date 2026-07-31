//! `SagaCompletionHandler` — holds the *same* injected saga instance as `SagaEventHandler`.

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_handler::{ExecutionRequest as HandlerExecutionRequest, Handler, HandlerError};
use edge_application_saga::{Saga, SagaIsCompleteRequest};
use parking_lot::Mutex;

use crate::api::{OrderSaga, SagaCompletionRequest, SagaCompletionResponse};

/// Holds the *same* injected saga instance as `SagaEventHandler`.
pub struct SagaCompletionHandler {
    pub saga: Arc<Mutex<OrderSaga>>,
}

#[async_trait]
impl Handler for SagaCompletionHandler {
    type Request = SagaCompletionRequest;
    type Response = SagaCompletionResponse;

    async fn execute(
        &self,
        _req: HandlerExecutionRequest<'_, SagaCompletionRequest>,
    ) -> Result<SagaCompletionResponse, HandlerError> {
        tracing::info!("[1] SagaCompletionHandler::execute — delegating to its own injected saga");
        let complete = self
            .saga
            .lock()
            .is_complete(SagaIsCompleteRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .complete;
        tracing::info!("[2] SagaCompletionHandler::execute — saga reports complete = {complete}");
        Ok(SagaCompletionResponse { complete })
    }
}
