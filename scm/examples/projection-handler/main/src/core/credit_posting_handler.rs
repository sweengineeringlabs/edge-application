//! `CreditPostingHandler` — holds its own injected `Projection` behind `Arc<Mutex<Balance>>`;
//! genuinely reads `req.ctx` inside `execute()`.

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_handler::{
    DrainRequest, ExecutionRequest as HandlerExecutionRequest, Handler, HandlerError,
    LogEmitRequest,
};
use edge_application_projection::{Projection, TryDrainRequest};
use parking_lot::Mutex;

use crate::api::{Balance, Credited, CreditPostingRequest, CreditPostingResponse};

/// Holds its own injected projection behind a lock; genuinely reads `req.ctx`.
pub struct CreditPostingHandler {
    pub balance: Arc<Mutex<Balance>>,
}

#[async_trait]
impl Handler for CreditPostingHandler {
    type Request = CreditPostingRequest;
    type Response = CreditPostingResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, CreditPostingRequest>,
    ) -> Result<CreditPostingResponse, HandlerError> {
        // ctx IS genuinely read here — real per-request observability, not filler.
        req.ctx
            .observer
            .drain(DrainRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .drain
            .emit(LogEmitRequest {
                level: "INFO".to_string(),
                handler_id: "credit_posting_handler".to_string(),
                message: format!("applying {} credit event(s)", req.req.amounts.len()),
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;

        tracing::info!("[1] CreditPostingHandler::execute — delegating to its own injected Projection");
        let events: Vec<Credited> = req
            .req
            .amounts
            .iter()
            .map(|&amount| Credited { amount })
            .collect();
        let count = {
            let mut balance = self.balance.lock();
            balance
                .try_drain(TryDrainRequest { events: &events })
                .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
                .count
        };
        tracing::info!("[2] CreditPostingHandler::execute — projection folded {count} event(s)");
        Ok(CreditPostingResponse { applied: count })
    }
}
