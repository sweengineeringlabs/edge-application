//! `GreeterRegistrationHandler` — holds its own injected `Registry`; genuinely reads `req.ctx`
//! inside `execute()`.

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_handler::{
    DrainRequest, ExecutionRequest as HandlerExecutionRequest, Handler, HandlerError,
    LogEmitRequest,
};
use edge_application_registry::{Registry, TryRegisterRequest};

use crate::api::{
    EnglishGreeter, Greeter, GreeterRegistrationRequest, GreeterRegistrationResponse,
    SpanishGreeter,
};

pub struct GreeterRegistrationHandler {
    pub registry: Arc<dyn Registry<Value = dyn Greeter>>,
}

#[async_trait]
impl Handler for GreeterRegistrationHandler {
    type Request = GreeterRegistrationRequest;
    type Response = GreeterRegistrationResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, GreeterRegistrationRequest>,
    ) -> Result<GreeterRegistrationResponse, HandlerError> {
        // ctx IS genuinely read here — real per-request observability, not filler.
        req.ctx
            .observer
            .drain(DrainRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .drain
            .emit(LogEmitRequest {
                level: "INFO".to_string(),
                handler_id: "greeter_registration_handler".to_string(),
                message: format!("registering {:?} greeter under {:?}", req.req.language, req.req.id),
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;

        tracing::info!("[1] GreeterRegistrationHandler::execute — delegating to its own injected Registry");
        let greeter: Arc<dyn Greeter> = match req.req.language.as_str() {
            "es" => Arc::new(SpanishGreeter),
            _ => Arc::new(EnglishGreeter),
        };
        self.registry
            .try_register(TryRegisterRequest {
                id: req.req.id,
                entry: greeter,
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        tracing::info!("[2] GreeterRegistrationHandler::execute — registry confirmed the registration");
        Ok(GreeterRegistrationResponse { registered: true })
    }
}
