//! `OrderPersistenceRequest` — `OrderPersistenceHandler`'s `Self::Request`.

#[derive(Debug, Clone)]
pub struct OrderPersistenceRequest {
    pub order_id: String,
    pub item: String,
    pub quantity: u32,
}
impl edge_application_base::Request for OrderPersistenceRequest {}
