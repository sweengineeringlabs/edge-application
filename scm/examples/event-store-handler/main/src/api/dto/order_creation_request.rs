//! `OrderCreationRequest` — `OrderCreationHandler`'s `Self::Request`.

#[derive(Debug, Clone)]
pub struct OrderCreationRequest {
    pub order_id: String,
    pub item: String,
}
impl edge_application_base::Request for OrderCreationRequest {}
