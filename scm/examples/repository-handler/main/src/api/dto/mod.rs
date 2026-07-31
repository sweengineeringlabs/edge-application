//! DTOs for the two `Handler`s wired to a shared `Repository<Order, String>`.

mod order_lookup_request;
mod order_lookup_response;
mod order_persistence_request;
mod order_persistence_response;

pub use order_lookup_request::OrderLookupRequest;
pub use order_lookup_response::OrderLookupResponse;
pub use order_persistence_request::OrderPersistenceRequest;
pub use order_persistence_response::OrderPersistenceResponse;
