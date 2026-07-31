//! `Query` theme — read operations that never mutate domain state.

pub mod direct_query_bus;
pub mod dto;
pub mod errors;
pub mod logging_query_bus;
pub mod noop;
pub mod traits;

pub use direct_query_bus::DirectQueryBus;
pub use dto::{
    QueryDispatchRequest, QueryExecuteRequest, QueryNameRequest, QueryNameResponse,
    QueryResultResponse,
};
pub use errors::QueryError;
pub use logging_query_bus::LoggingQueryBus;
pub use noop::{NoopQuery, NoopQueryBus};
pub use traits::{Query, QueryBus};
