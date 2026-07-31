//! `PolicyIdentityHandler` — holds the *same* injected `Policy` instance as
//! `WithdrawalPolicyHandler`.

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_handler::{ExecutionRequest as HandlerExecutionRequest, Handler, HandlerError};
use edge_application_policy::{Policy, PolicyNameRequest};

use crate::api::{PolicyIdentityRequest, PolicyIdentityResponse};

pub struct PolicyIdentityHandler {
    pub policy: Arc<dyn Policy<Input = u64>>,
}

#[async_trait]
impl Handler for PolicyIdentityHandler {
    type Request = PolicyIdentityRequest;
    type Response = PolicyIdentityResponse;

    async fn execute(
        &self,
        _req: HandlerExecutionRequest<'_, PolicyIdentityRequest>,
    ) -> Result<PolicyIdentityResponse, HandlerError> {
        tracing::info!("[1] PolicyIdentityHandler::execute — delegating to its own injected Policy");
        let name = self
            .policy
            .name(PolicyNameRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .name;
        tracing::info!("[2] PolicyIdentityHandler::execute — policy reports name = {name:?}");
        Ok(PolicyIdentityResponse {
            name: name.to_string(),
        })
    }
}
