//! `GreeterResolutionHandler` — holds the *same* injected `Registry` instance as
//! `GreeterRegistrationHandler`.

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_handler::{ExecutionRequest as HandlerExecutionRequest, Handler, HandlerError};
use edge_application_registry::{Registry, RegistryLookupRequest};

use crate::api::{Greeter, GreeterResolutionRequest, GreeterResolutionResponse};

pub struct GreeterResolutionHandler {
    pub registry: Arc<dyn Registry<Value = dyn Greeter>>,
}

#[async_trait]
impl Handler for GreeterResolutionHandler {
    type Request = GreeterResolutionRequest;
    type Response = GreeterResolutionResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, GreeterResolutionRequest>,
    ) -> Result<GreeterResolutionResponse, HandlerError> {
        tracing::info!("[1] GreeterResolutionHandler::execute — delegating to its own injected Registry");
        let resolved = self
            .registry
            .get(RegistryLookupRequest { id: req.req.id })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .entry;
        let greeting = resolved.map(|greeter| greeter.greet(&req.req.name));
        tracing::info!("[2] GreeterResolutionHandler::execute — registry returned {greeting:?}");
        Ok(GreeterResolutionResponse { greeting })
    }
}
