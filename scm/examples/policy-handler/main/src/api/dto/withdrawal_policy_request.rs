//! `WithdrawalPolicyRequest` — `WithdrawalPolicyHandler`'s `Self::Request`.

#[derive(Debug, Clone, Copy)]
pub struct WithdrawalPolicyRequest {
    pub amount: u64,
}
impl edge_application_base::Request for WithdrawalPolicyRequest {}
