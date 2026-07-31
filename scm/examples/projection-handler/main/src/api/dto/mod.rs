//! DTOs for the two `Handler`s wired to a shared `Projection<Credited, u64>`.

mod balance_lookup_request;
mod balance_lookup_response;
mod credit_posting_request;
mod credit_posting_response;

pub use balance_lookup_request::BalanceLookupRequest;
pub use balance_lookup_response::BalanceLookupResponse;
pub use credit_posting_request::CreditPostingRequest;
pub use credit_posting_response::CreditPostingResponse;
