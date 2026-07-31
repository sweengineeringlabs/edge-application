//! `BalanceLookupHandler` — holds the *same* injected `Projection` instance as
//! `CreditPostingHandler`, one projection, multiple handlers, each reaching it through their own
//! constructor-injected field.

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_handler::{ExecutionRequest as HandlerExecutionRequest, Handler, HandlerError};
use edge_application_projection::{Projection, ProjectionReadModelRequest};
use parking_lot::Mutex;

use crate::api::{Balance, BalanceLookupRequest, BalanceLookupResponse};

/// Holds the *same* injected projection instance as `CreditPostingHandler`.
pub struct BalanceLookupHandler {
    pub balance: Arc<Mutex<Balance>>,
}

#[async_trait]
impl Handler for BalanceLookupHandler {
    type Request = BalanceLookupRequest;
    type Response = BalanceLookupResponse;

    async fn execute(
        &self,
        _req: HandlerExecutionRequest<'_, BalanceLookupRequest>,
    ) -> Result<BalanceLookupResponse, HandlerError> {
        tracing::info!("[1] BalanceLookupHandler::execute — delegating to its own injected Projection");
        let total = *self
            .balance
            .lock()
            .read_model(ProjectionReadModelRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .read_model;
        tracing::info!("[2] BalanceLookupHandler::execute — projection reports total = {total}");
        Ok(BalanceLookupResponse { total })
    }
}
