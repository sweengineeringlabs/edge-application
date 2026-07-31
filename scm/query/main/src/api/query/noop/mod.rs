//! Zero-sized noop marker types for testing and bring-up.

pub mod noop_query;
pub mod noop_query_bus;

pub use noop_query::NoopQuery;
pub use noop_query_bus::NoopQueryBus;
