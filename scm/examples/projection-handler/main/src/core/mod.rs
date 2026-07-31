//! Core layer — `Handler` implementations wired to a shared `Projection<Credited, u64>`.

mod balance_lookup_handler;
mod credit_posting_handler;

pub use balance_lookup_handler::BalanceLookupHandler;
pub use credit_posting_handler::CreditPostingHandler;
