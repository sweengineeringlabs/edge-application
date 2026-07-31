//! API layer — DTOs and policy rules for the two `Handler`s wired to a shared `Policy<Input = u64>`.

mod dto;
mod max_amount_policy;
mod min_amount_policy;

pub use dto::{
    PolicyIdentityRequest, PolicyIdentityResponse, WithdrawalPolicyRequest, WithdrawalPolicyResponse,
};
pub use max_amount_policy::MaxAmountPolicy;
pub use min_amount_policy::MinAmountPolicy;
