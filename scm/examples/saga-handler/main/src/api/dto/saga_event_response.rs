//! `SagaEventResponse` — `SagaEventHandler`'s `Self::Response`.

#[derive(Debug, Clone, Copy)]
pub struct SagaEventResponse {
    pub commands_dispatched: usize,
}
impl edge_application_base::Response for SagaEventResponse {}
