//! API layer — DTOs for the two `Handler`s wired to a shared `OrderSaga`.

mod dto;
mod order_paid;
mod order_saga;
mod ship_order_command;

pub use dto::{SagaCompletionRequest, SagaCompletionResponse, SagaEventRequest, SagaEventResponse};
pub use order_paid::OrderPaid;
pub use order_saga::OrderSaga;
pub use ship_order_command::ShipOrderCommand;
