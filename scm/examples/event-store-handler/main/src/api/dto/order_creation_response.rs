//! `OrderCreationResponse` — `OrderCreationHandler`'s `Self::Response`.

#[derive(Debug, Clone, Copy)]
pub struct OrderCreationResponse {
    pub sequence: u64,
}
impl edge_application_base::Response for OrderCreationResponse {}
