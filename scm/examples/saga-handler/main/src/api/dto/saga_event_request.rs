//! `SagaEventRequest` — `SagaEventHandler`'s `Self::Request`.

#[derive(Debug, Clone)]
pub struct SagaEventRequest {
    pub order_id: String,
}
impl edge_application_base::Request for SagaEventRequest {}
