//! `OrderLookupRequest` — `OrderLookupHandler`'s `Self::Request`.

#[derive(Debug, Clone)]
pub struct OrderLookupRequest {
    pub order_id: String,
}
impl edge_application_base::Request for OrderLookupRequest {}
