//! `CreditPostingRequest` — `CreditPostingHandler`'s `Self::Request`.

#[derive(Debug, Clone)]
pub struct CreditPostingRequest {
    pub amounts: Vec<u64>,
}
impl edge_application_base::Request for CreditPostingRequest {}
