//! DTOs for the two `Handler`s wired to a shared `EventStore<OrderCreated>`.

mod order_creation_request;
mod order_creation_response;
mod order_history_request;
mod order_history_response;

pub use order_creation_request::OrderCreationRequest;
pub use order_creation_response::OrderCreationResponse;
pub use order_history_request::OrderHistoryRequest;
pub use order_history_response::OrderHistoryResponse;
