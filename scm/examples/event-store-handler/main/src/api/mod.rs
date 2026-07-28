//! API layer — DTOs for the two `Handler`s wired to a shared `EventStore<OrderCreated>`.

mod dto;
mod order_created;

pub use dto::{
    OrderCreationRequest, OrderCreationResponse, OrderHistoryRequest, OrderHistoryResponse,
};
pub use order_created::OrderCreated;
