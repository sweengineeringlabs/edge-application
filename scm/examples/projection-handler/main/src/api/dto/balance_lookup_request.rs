//! `BalanceLookupRequest` — `BalanceLookupHandler`'s `Self::Request`.

#[derive(Debug, Clone, Copy, Default)]
pub struct BalanceLookupRequest;
impl edge_application_base::Request for BalanceLookupRequest {}
