//! API layer — DTOs for the two `Handler`s wired to a shared `Projection<Credited, u64>`.

mod balance;
mod credited;
mod dto;

pub use balance::Balance;
pub use credited::Credited;
pub use dto::{
    BalanceLookupRequest, BalanceLookupResponse, CreditPostingRequest, CreditPostingResponse,
};
