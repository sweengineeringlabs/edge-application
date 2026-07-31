//! DTOs for the two `Handler`s wired to a shared `Policy<Input = u64>`.

mod policy_identity_request;
mod policy_identity_response;
mod withdrawal_policy_request;
mod withdrawal_policy_response;

pub use policy_identity_request::PolicyIdentityRequest;
pub use policy_identity_response::PolicyIdentityResponse;
pub use withdrawal_policy_request::WithdrawalPolicyRequest;
pub use withdrawal_policy_response::WithdrawalPolicyResponse;
