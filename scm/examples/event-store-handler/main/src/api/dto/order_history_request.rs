//! `OrderHistoryRequest` — `OrderHistoryHandler`'s `Self::Request`.

#[derive(Debug, Clone)]
pub struct OrderHistoryRequest {
    pub order_id: String,
}
impl edge_application_base::Request for OrderHistoryRequest {}
