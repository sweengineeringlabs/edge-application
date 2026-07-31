//! `WithdrawalPolicyHandler` — holds its own injected `Policy`; genuinely reads `req.ctx`
//! inside `execute()`.

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_handler::{
    DrainRequest, ExecutionRequest as HandlerExecutionRequest, Handler, HandlerError,
    LogEmitRequest,
};
use edge_application_policy::{Policy, PolicyEvaluateRequest};

use crate::api::{WithdrawalPolicyRequest, WithdrawalPolicyResponse};

pub struct WithdrawalPolicyHandler {
    pub policy: Arc<dyn Policy<Input = u64>>,
}

#[async_trait]
impl Handler for WithdrawalPolicyHandler {
    type Request = WithdrawalPolicyRequest;
    type Response = WithdrawalPolicyResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, WithdrawalPolicyRequest>,
    ) -> Result<WithdrawalPolicyResponse, HandlerError> {
        // ctx IS genuinely read here — real per-request observability, not filler.
        req.ctx
            .observer
            .drain(DrainRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .drain
            .emit(LogEmitRequest {
                level: "INFO".to_string(),
                handler_id: "withdrawal_policy_handler".to_string(),
                message: format!("evaluating withdrawal of {}", req.req.amount),
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;

        tracing::info!("[1] WithdrawalPolicyHandler::execute — delegating to its own injected Policy");
        self.policy
            .evaluate(PolicyEvaluateRequest {
                input: &req.req.amount,
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        tracing::info!("[2] WithdrawalPolicyHandler::execute — policy allowed the withdrawal");
        Ok(WithdrawalPolicyResponse { allowed: true })
    }
}
