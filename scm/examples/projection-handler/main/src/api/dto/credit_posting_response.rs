//! `CreditPostingResponse` — `CreditPostingHandler`'s `Self::Response`.

#[derive(Debug, Clone, Copy)]
pub struct CreditPostingResponse {
    pub applied: usize,
}
impl edge_application_base::Response for CreditPostingResponse {}
