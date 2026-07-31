//! `WithdrawalPolicyResponse` — `WithdrawalPolicyHandler`'s `Self::Response`.

#[derive(Debug, Clone, Copy)]
pub struct WithdrawalPolicyResponse {
    pub allowed: bool,
}
impl edge_application_base::Response for WithdrawalPolicyResponse {}
