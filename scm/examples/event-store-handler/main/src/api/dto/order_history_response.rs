//! `OrderHistoryResponse` — `OrderHistoryHandler`'s `Self::Response`.

#[derive(Debug, Clone)]
pub struct OrderHistoryResponse {
    pub items: Vec<String>,
}
impl edge_application_base::Response for OrderHistoryResponse {}
