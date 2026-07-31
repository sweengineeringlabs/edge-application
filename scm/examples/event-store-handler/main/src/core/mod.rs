//! Core layer — `Handler` implementations wired to a shared `EventStore<OrderCreated>`.

mod order_creation_handler;
mod order_history_handler;

pub use order_creation_handler::OrderCreationHandler;
pub use order_history_handler::OrderHistoryHandler;
