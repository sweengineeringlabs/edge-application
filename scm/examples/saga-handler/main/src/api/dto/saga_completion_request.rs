//! `SagaCompletionRequest` — `SagaCompletionHandler`'s `Self::Request`.

#[derive(Debug, Clone, Copy, Default)]
pub struct SagaCompletionRequest;
impl edge_application_base::Request for SagaCompletionRequest {}
