//! API-layer type for the logging query bus.

use std::sync::Arc;

use crate::api::query::traits::QueryBus;

/// A [`QueryBus`] decorator that records each dispatched query and its
/// outcome via `tracing`.
///
/// The concrete `QueryBus` implementation lives in `core::query::logging_query_bus`.
pub struct LoggingQueryBus<R> {
    /// The inner bus that receives the delegated dispatch calls.
    pub inner: Arc<dyn QueryBus<Result = R>>,
}
