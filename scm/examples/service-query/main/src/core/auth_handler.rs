//! `AuthHandler` — the port other domain logic calls. Holds its own `QueryBus`, injected once at
//! construction.

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_handler::{
    DrainRequest, ExecutionRequest as HandlerExecutionRequest, Handler, HandlerError, LogEmitRequest,
};
use edge_application_query::{QueryBus, QueryDispatchRequest, QueryError};

use crate::api::{IsLoggedInRequest, IsLoggedInResponse, LoginStatusQuery};

pub struct AuthHandler {
    pub query_bus: Arc<dyn QueryBus<Result = bool>>,
}

#[async_trait]
impl Handler for AuthHandler {
    type Request = IsLoggedInRequest;
    type Response = IsLoggedInResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, IsLoggedInRequest>,
    ) -> Result<IsLoggedInResponse, HandlerError> {
        // ctx IS genuinely read here — real per-request observability, not filler.
        req.ctx
            .observer
            .drain(DrainRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .drain
            .emit(LogEmitRequest {
                level: "INFO".to_string(),
                handler_id: "auth_handler".to_string(),
                message: format!("checking login for {:?}", req.req.session_token),
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;

        tracing::info!("[1] AuthHandler::execute — delegating to its own injected QueryBus");
        let result = self
            .query_bus
            .dispatch(QueryDispatchRequest {
                query: Box::new(LoginStatusQuery {
                    session_token: req.req.session_token,
                }),
            })
            .await
            .map_err(|e: QueryError| HandlerError::ExecutionFailed(e.to_string()))?;
        tracing::info!(
            "[3] AuthHandler::execute — got {} back from the query bus",
            result.result
        );
        Ok(IsLoggedInResponse {
            logged_in: result.result,
        })
    }
}
