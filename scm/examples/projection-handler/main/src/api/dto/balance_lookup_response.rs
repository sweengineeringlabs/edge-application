//! `BalanceLookupResponse` — `BalanceLookupHandler`'s `Self::Response`.

#[derive(Debug, Clone, Copy)]
pub struct BalanceLookupResponse {
    pub total: u64,
}
impl edge_application_base::Response for BalanceLookupResponse {}
