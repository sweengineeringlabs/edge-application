//! `SagaCompletionResponse` — `SagaCompletionHandler`'s `Self::Response`.

#[derive(Debug, Clone, Copy)]
pub struct SagaCompletionResponse {
    pub complete: bool,
}
impl edge_application_base::Response for SagaCompletionResponse {}
