//! # edge-domain-query
//!
//! The `Query` port contract — CQRS read-side with QueryBus and DirectQueryBus.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use api::DirectQueryBus;
pub use api::LoggingQueryBus;
pub use api::NoopQuery;
pub use api::NoopQueryBus;
pub use api::QueryDispatchRequest;
pub use api::QueryError;
pub use api::QueryExecuteRequest;
pub use api::QueryNameRequest;
pub use api::QueryNameResponse;
pub use api::QueryResultResponse;
pub use saf::Query;
pub use saf::QueryBus;
pub use saf::QUERY_BUS_SVC;
pub use saf::QUERY_BUS_SVC_FACTORY;
pub use saf::QUERY_SVC;
pub use saf::QUERY_SVC_FACTORY;
