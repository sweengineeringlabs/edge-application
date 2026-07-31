//! `OrderLookupResponse` — `OrderLookupHandler`'s `Self::Response`.

use crate::api::order::Order;

#[derive(Debug, Clone)]
pub struct OrderLookupResponse {
    pub order: Option<Order>,
}
impl edge_application_base::Response for OrderLookupResponse {}
